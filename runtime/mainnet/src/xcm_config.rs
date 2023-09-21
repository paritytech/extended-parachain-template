use super::{
	AccountId, AllPalletsWithSystem, Assets, Balance, Balances, ParachainInfo, ParachainSystem,
	PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, WeightToFee, XcmpQueue,
};
use core::{marker::PhantomData, ops::ControlFlow};
use frame_support::{
	match_types, parameter_types,
	traits::{ConstU32, Everything, Nothing, PalletInfoAccess, ProcessMessageError},
	weights::Weight,
};
use frame_system::EnsureRoot;
use pallet_xcm::XcmPassthrough;
use polkadot_parachain_primitives::primitives::Sibling;
use polkadot_runtime_common::impls::ToAuthor;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowExplicitUnpaidExecutionFrom, AllowTopLevelPaidExecutionFrom,
	CreateMatcher, CurrencyAdapter, EnsureXcmOrigin, FixedWeightBounds, FungiblesAdapter,
	IsConcrete, MatchXcm, NativeAsset, NoChecking, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId,
	UsingComponents, WithComputedOrigin, WithUniqueTopic,
};
use xcm_executor::traits::Properties;
use xcm_executor::{traits::ShouldExecute, XcmExecutor};

parameter_types! {
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: Option<NetworkId> = None;
	pub SelfReserve: MultiLocation = MultiLocation { parents: 0, interior: Here };
	pub PlaceholderAccount: AccountId = PolkadotXcm::check_account();
	pub AssetsPalletLocation: MultiLocation =
		PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub UniversalLocation: InteriorMultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// `AssetId/Balancer` converter for `TrustBackedAssets`
pub type TrustBackedAssetsConvertedConcreteId =
	assets_common::TrustBackedAssetsConvertedConcreteId<AssetsPalletLocation, Balance>;

/// Means for transacting assets on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<SelfReserve>,
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

/// Means for transacting assets besides the native currency on this chain.
pub type LocalFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	Assets,
	// Use this currency when it is a fungible asset matching the given location or name:
	TrustBackedAssetsConvertedConcreteId,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Assets`.
	NoChecking,
	// We don't track any teleports of `Assets`, but a placeholder account is provided due to trait
	// bounds.
	PlaceholderAccount,
>;

/// Means for transacting assets on this chain.
pub type AssetTransactors = (LocalAssetTransactor, LocalFungiblesTransactor);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `RuntimeOrigin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
}

//TODO: move DenyThenTry to polkadot's xcm module.
/// Deny executing the xcm message if it matches any of the Deny filter regardless of anything else.
/// If it passes the Deny, and matches one of the Allow cases then it is let through.
pub struct DenyThenTry<Deny, Allow>(PhantomData<Deny>, PhantomData<Allow>)
where
	Deny: ShouldExecute,
	Allow: ShouldExecute;

impl<Deny, Allow> ShouldExecute for DenyThenTry<Deny, Allow>
where
	Deny: ShouldExecute,
	Allow: ShouldExecute,
{
	fn should_execute<RuntimeCall>(
		origin: &MultiLocation,
		instructions: &mut [Instruction<RuntimeCall>],
		max_weight: Weight,
		properties: &mut Properties,
	) -> Result<(), ProcessMessageError> {
		Deny::should_execute(origin, instructions, max_weight, properties)?;
		Allow::should_execute(origin, instructions, max_weight, properties)
	}
}

// See issue <https://github.com/paritytech/polkadot/issues/5233>
pub struct DenyReserveTransferToRelayChain;
impl ShouldExecute for DenyReserveTransferToRelayChain {
	fn should_execute<RuntimeCall>(
		origin: &MultiLocation,
		instructions: &mut [Instruction<RuntimeCall>],
		_max_weight: Weight,
		_properties: &mut Properties,
	) -> Result<(), ProcessMessageError> {
		instructions.matcher().match_next_inst_while(
			|_| true,
			|inst| match inst {
				InitiateReserveWithdraw {
					reserve: MultiLocation { parents: 1, interior: Here },
					..
				}
				| DepositReserveAsset {
					dest: MultiLocation { parents: 1, interior: Here }, ..
				}
				| TransferReserveAsset {
					dest: MultiLocation { parents: 1, interior: Here }, ..
				} => {
					Err(ProcessMessageError::Unsupported) // Deny
				},
				// An unexpected reserve transfer has arrived from the Relay Chain. Generally,
				// `IsReserve` should not allow this, but we just log it here.
				ReserveAssetDeposited { .. }
					if matches!(origin, MultiLocation { parents: 1, interior: Here }) =>
				{
					log::warn!(
						target: "xcm::barrier",
						"Unexpected ReserveAssetDeposited from the Relay Chain",
					);
					Ok(ControlFlow::Continue(()))
				},
				_ => Ok(ControlFlow::Continue(())),
			},
		)?;

		// Permit everything else
		Ok(())
	}
}

pub type Barrier = TrailingSetTopicAsId<
	DenyThenTry<
		DenyReserveTransferToRelayChain,
		(
			TakeWeightCredit,
			WithComputedOrigin<
				(
					AllowTopLevelPaidExecutionFrom<Everything>,
					// Parent and its exec plurality get free execution
					AllowExplicitUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
				),
				UniversalLocation,
				ConstU32<8>,
			>,
		),
	>,
>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	// Teleporting is disabled.
	type IsTeleporter = ();
	type Aliasers = Nothing;
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = UsingComponents<WeightToFee, SelfReserve, AccountId, Balances, ToAuthor<Runtime>>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetLocker = ();
	type AssetExchanger = ();
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = WithUniqueTopic<(
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, (), ()>,
	// and XCMP to communicate with the sibling chains.
	XcmpQueue,
)>;

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	// We disallow users to send arbitrary XCMs from this chain.
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type AdminOrigin = EnsureRoot<AccountId>;
	type XcmRouter = XcmRouter;
	// We disallow users to execute arbitrary XCMs from this chain.
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	// Disable dispatchable execute on the XCM pallet.
	// NOTE: This needs to be `Everything` for local testing.
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	// No teleportation is allowed.
	type XcmTeleportFilter = Nothing;
	// All reserve transfers are allowed.
	type XcmReserveTransferFilter = Everything;
	// Use (conservative) bounds on estimating XCM execution on this chain.
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;

	// Override for AdvertisedXcmVersion default
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}
