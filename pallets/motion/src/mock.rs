use super::*;
pub(crate) use crate as pallet_motion;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU32, ConstU64},
	weights::Weight,
};
use frame_system::limits::BlockWeights;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage, Perbill,
};

type Block = frame_system::mocking::MockBlock<Test>;

parameter_types! {
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::default();
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system::{Pallet, Call, Event<T>},

		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
		Motion: pallet_motion,
	}
);

impl frame_system::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type FreezeIdentifier = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
	type RuntimeFreezeReason = RuntimeFreezeReason;
}

parameter_types! {
	pub CouncilMotionDuration: u64 = 7;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
}

pub type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = frame_system::EnsureRoot<u64>;
	type MaxProposalWeight = MaxProposalWeight;
}

impl pallet_motion::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type SimpleMajorityOrigin =
		pallet_collective::EnsureProportionAtLeast<u64, CouncilCollective, 1, 2>;
	type SuperMajorityOrigin =
		pallet_collective::EnsureProportionAtLeast<u64, CouncilCollective, 2, 3>;
	type UnanimousOrigin = pallet_collective::EnsureProportionAtLeast<u64, CouncilCollective, 1, 1>;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = RuntimeGenesisConfig {
		balances: pallet_balances::GenesisConfig::<Test> {
			balances: vec![(1, 10), (2, 20), (3, 30), (4, 40), (5, 50)],
		},
		council: pallet_collective::GenesisConfig {
			members: vec![1, 2, 3, 4],
			phantom: Default::default(),
		},
	}
	.build_storage()
	.unwrap()
	.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
