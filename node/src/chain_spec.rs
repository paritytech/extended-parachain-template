use cumulus_primitives_core::ParaId;
use runtime_common::{Balance, AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	AccountId32,
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type MainChainSpec =
	sc_service::GenericChainSpec<mainnet_runtime::RuntimeGenesisConfig, Extensions>;

/// Specialized `ChainSpec` for the development parachain runtime.
pub type DevnetChainSpec =
	sc_service::GenericChainSpec<devnet_runtime::RuntimeGenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

const PARA_ID: u32 = 2000;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn mainnet_session_keys(keys: AuraId) -> mainnet_runtime::SessionKeys {
	mainnet_runtime::SessionKeys { aura: keys }
}

pub fn devnet_session_keys(keys: AuraId) -> devnet_runtime::SessionKeys {
	devnet_runtime::SessionKeys { aura: keys }
}

/// Generate a multisig key from a given `authority_set` and a `threshold`
/// Used for generating a multisig to use as sudo key on mainnet
pub fn get_multisig_sudo_key(mut authority_set: Vec<AccountId32>, threshold: u16) -> AccountId {
	assert!(threshold > 0, "Threshold for sudo multisig cannot be 0");
	assert!(!authority_set.is_empty(), "Sudo authority set cannot be empty");
	assert!(
		authority_set.len() >= threshold.into(),
		"Threshold must be less than or equal to authority set members"
	);
	// Sorting is done to deterministically order the multisig set
	// So that a single authority set (A, B, C) may generate only a single unique multisig key
	// Otherwise, (B, A, C) or (C, A, B) could produce different keys and cause chaos
	authority_set.sort();

	// Define a multisig threshold for `threshold / authoriy_set.len()` members
	pallet_multisig::Pallet::<mainnet_runtime::Runtime>::multi_account_id(
		&authority_set[..],
		threshold,
	)
}

pub mod devnet {
	use super::*;
	pub fn development_config() -> DevnetChainSpec {
		// Give your base currency a unit name and decimal places
		let mut properties = sc_chain_spec::Properties::new();
		properties.insert("tokenSymbol".into(), "DEV".into());
		properties.insert("tokenDecimals".into(), 12.into());
		properties.insert("ss58Format".into(), 42.into());

		DevnetChainSpec::builder(
			devnet_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
			Extensions {
				relay_chain: "rococo-local".into(),
				// You MUST set this to the correct network!
				para_id: PARA_ID,
			},
		)
		.with_name("Development")
		.with_id("dev")
		.with_chain_type(ChainType::Development)
		.with_genesis_config_patch(testnet_genesis(
			vec![
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_collator_keys_from_seed("Bob"),
				),
			],
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
			PARA_ID.into(),
		))
		.with_properties(properties)
		.build()
	}

	pub fn local_testnet_config() -> DevnetChainSpec {
		// Give your base currency a unit name and decimal places
		let mut properties = sc_chain_spec::Properties::new();
		properties.insert("tokenSymbol".into(), "UNIT".into());
		properties.insert("tokenDecimals".into(), 12.into());
		properties.insert("ss58Format".into(), 42.into());

		DevnetChainSpec::builder(
			devnet_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
			Extensions {
				relay_chain: "rococo-local".into(),
				// You MUST set this to the correct network!
				para_id: PARA_ID,
			},
		)
		.with_name("Development")
		.with_id("dev")
		.with_chain_type(ChainType::Development)
		.with_genesis_config_patch(testnet_genesis(
			// initial collators.
			vec![
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_collator_keys_from_seed("Bob"),
				),
			],
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
			PARA_ID.into(),
		))
		.with_protocol_id("template-local")
		.with_properties(properties)
		.build()
	}

	fn testnet_genesis(
		invulnerables: Vec<(AccountId, AuraId)>,
		endowed_accounts: Vec<AccountId>,
		root_key: Option<AccountId>,
		id: ParaId,
	) -> serde_json::Value {
		use devnet_runtime::{BalancesConfig, EXISTENTIAL_DEPOSIT};
		let alice = get_from_seed::<sr25519::Public>("Alice");
		let bob = get_from_seed::<sr25519::Public>("Bob");
		serde_json::json!( {
				"balances": BalancesConfig { balances : endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect::<Vec<_>>() },
				// Configure two assets ALT1 & ALT2 with two owners, alice and bob respectively
				"assets": {
					"assets": vec![
						(1, Into::<AccountId>::into(alice), true, 100_000_000_000 as Balance),
						(2, bob.into(), true, 100_000_000_000),
					],
					// Genesis metadata: Vec<(id, name, symbol, decimals)>
					"metadata": vec![
						(1, "asset-1".as_bytes(), "ALT1".as_bytes(), 10),
						(2, "asset-2".as_bytes(), "ALT2".as_bytes(), 10),
					],
					// Genesis accounts: Vec<(id, account_id, balance)>
					"accounts": vec![
						(1, Into::<AccountId>::into(alice), 500_000_000_000 as Balance),
						(2, bob.into(), 500_000_000_000),
					],
				},
				"parachainInfo": { "parachainId": id },
				"collatorSelection": {
					"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
					"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
				},
				"session": {
					"keys": invulnerables
						.into_iter()
						.map(|(acc, aura)| {
							(
								acc.clone(),               // account id
								acc,                       // validator id
								devnet_session_keys(aura), // session keys
							)
						})
						.collect::<Vec<_>>(),
				},
				"sudo" : { "key" : root_key },
				"council": {
					"members": endowed_accounts.iter().take(4).cloned().collect::<Vec<_>>(),
				},
				"polkadotXcm": { "safeXcmVersion": Some(SAFE_XCM_VERSION) },
			}
		)
	}
}

pub mod mainnet {
	use super::*;
	pub fn development_config() -> MainChainSpec {
		// Give your base currency a unit name and decimal places
		let mut properties = sc_chain_spec::Properties::new();
		properties.insert("tokenSymbol".into(), "UNIT".into());
		properties.insert("tokenDecimals".into(), 12.into());
		properties.insert("ss58Format".into(), 42.into());

		MainChainSpec::builder(
			mainnet_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
			Extensions {
				relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
				para_id: PARA_ID,
			},
		)
		// Name
		.with_name("Mainnet Development")
		// ID
		.with_id("main_dev")
		.with_chain_type(ChainType::Custom("Mainnet Development".to_string()))
		.with_genesis_config_patch(mainnet_genesis(
			// initial collators.
			vec![
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_collator_keys_from_seed("Bob"),
				),
			],
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			// Example multisig sudo key configuration:
			// Configures 2/3 threshold multisig key
			// Note: For using this multisig key as a sudo key, each individual signatory must possess funds
			get_multisig_sudo_key(
				vec![
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
				],
				2,
			),
			PARA_ID.into(),
		))
		.with_properties(properties)
		.build()
	}

	pub fn local_testnet_config() -> MainChainSpec {
		// Give your base currency a unit name and decimal places
		let mut properties = sc_chain_spec::Properties::new();
		properties.insert("tokenSymbol".into(), "UNIT".into());
		properties.insert("tokenDecimals".into(), 12.into());
		properties.insert("ss58Format".into(), 42.into());

		MainChainSpec::builder(
			mainnet_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
			Extensions {
				relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
				para_id: PARA_ID,
			},
		)
		.with_name("Mainnet Local Testnet")
		.with_id("main_local_testnet")
		.with_chain_type(ChainType::Local)
		.with_genesis_config_patch(mainnet_genesis(
			// initial collators.
			vec![
				(
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_collator_keys_from_seed("Alice"),
				),
				(
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_collator_keys_from_seed("Bob"),
				),
			],
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			// Example multisig sudo key configuration:
			// Configures 2/3 threshold multisig key
			// Note: For using this multisig key as a sudo key, each individual signatory must possess funds
			get_multisig_sudo_key(
				vec![
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
				],
				2,
			),
			PARA_ID.into(),
		))
		.with_protocol_id("template-local")
		.with_properties(properties)
		.build()
	}

	fn mainnet_genesis(
		invulnerables: Vec<(AccountId, AuraId)>,
		endowed_accounts: Vec<AccountId>,
		root_key: AccountId,
		id: ParaId,
	) -> serde_json::Value {
		use mainnet_runtime::EXISTENTIAL_DEPOSIT;
		let alice = get_from_seed::<sr25519::Public>("Alice");
		let bob = get_from_seed::<sr25519::Public>("Bob");

		serde_json::json!({
				"balances": {
					"balances": endowed_accounts
						.iter()
						.cloned()
						// Fund sudo key for sending transactions
						.chain(std::iter::once(root_key.clone()))
						.map(|k| (k, 1 << 60))
						.collect::<Vec<_>>(),
				},
				// Configure two assets ALT1 & ALT2 with two owners, alice and bob respectively
				"assets": {
					"assets": vec![
						(1, Into::<AccountId>::into(alice), true, 100_000_000_000 as Balance),
						(2, bob.into(), true, 100_000_000_000),
					],
					// Genesis metadata: Vec<(id, name, symbol, decimals)>
					"metadata": vec![
						(1, "asset-1".as_bytes(), "ALT1".as_bytes(), 10),
						(2, "asset-2".as_bytes(), "ALT2".as_bytes(), 10),
					],
					// Genesis accounts: Vec<(id, account_id, balance)>
					"accounts": vec![
						(1, Into::<AccountId>::into(alice), 500_000_000_000 as Balance),
						(2, bob.into(), 500_000_000_000),
					],
				},
				"parachainInfo": {
					"parachainId": id,
				},
				"collatorSelection": {
					"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
					"candidacy_bond": EXISTENTIAL_DEPOSIT * 16,
				},
				"session": {
					"keys": invulnerables
						.into_iter()
						.map(|(acc, aura)| {
							(
								acc.clone(),                // account id
								acc,                        // validator id
								mainnet_session_keys(aura), // session keys
							)
						})
						.collect::<Vec<_>>(),
				},
				"sudo": { "key": Some(root_key) },
				"council": {
					"members": endowed_accounts.iter().take(4).cloned().collect::<Vec<_>>(),
				},
				"polkadotXcm": {
					"safeXcmVersion": Some(SAFE_XCM_VERSION),
				},
			}
		)
	}
}
