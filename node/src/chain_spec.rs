use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use selendra_runtime::{
	constants::{common::EXISTENTIAL_DEPOSIT, currency::UNIT as SEL},
	AccountId, AuraId, Signature,
};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<selendra_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
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
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> selendra_runtime::SessionKeys {
	selendra_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "PSEL".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			selendra_genesis(
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
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				2000.into(),
			)
		},
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("development"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "PSEL".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			selendra_genesis(
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
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				2000.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("local-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "westend-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

pub fn selendra_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "PSEL".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			selendra_genesis(
				// initial collators.
				vec![
					(
						// 5EZN96zxrpq7woimDxeTpHGRjPzM2CqRngjq8QUw2LNEELJz
						hex!("6e5436279b4f0c0c707f4fb5d7156b8dd81392c35718d1d54dc1eda24b76f73d")
							.into(),
						hex!("6e5436279b4f0c0c707f4fb5d7156b8dd81392c35718d1d54dc1eda24b76f73d")
							.unchecked_into(),
					),
					(
						// 5G41FZcv43L1YPJXK9u7o31yrceB3zimMc6TF2Y5bZJ4XV3E
						hex!("b0683e8a2a5009b1e6f845ae1b95d6512a6a57e07aa5f9b91c8286c2a3988f4e")
							.into(),
						hex!("b0683e8a2a5009b1e6f845ae1b95d6512a6a57e07aa5f9b91c8286c2a3988f4e")
							.unchecked_into(),
					),
				],
				vec![
					// 5EZN96zxrpq7woimDxeTpHGRjPzM2CqRngjq8QUw2LNEELJz
					hex!["6e5436279b4f0c0c707f4fb5d7156b8dd81392c35718d1d54dc1eda24b76f73d"].into(),
					// 5G41FZcv43L1YPJXK9u7o31yrceB3zimMc6TF2Y5bZJ4XV3E
					hex!["b0683e8a2a5009b1e6f845ae1b95d6512a6a57e07aa5f9b91c8286c2a3988f4e"].into(),
					// 5EhroJdJCDyNgVJ6JbEPCEkEsPRtAAsdEgDAS46Y8iAMEgQn
					hex!["74cea5f84fb9423bf664f555f7c4d6f58f1a62e8d6f20936a4a5eb8f81ee1114"].into(),
					// 5Eo5UWonGx75gTnEFRi6A3So5qfHGP4dL1ZFcHAc6fonYKKd
					hex!["78c98bf320b2b8105af1fbca9558f506a291367f378da3e9e1dfcc7ab9bbc951"].into(),
				],
				2000.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("local-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "westend-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

fn selendra_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> selendra_runtime::GenesisConfig {
	/// Initial issue 1_256_637_061 PSEL;
	const ENDOWMENT: u128 = 314159265 * SEL;

	selendra_runtime::GenesisConfig {
		system: selendra_runtime::SystemConfig {
			code: selendra_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		sudo: selendra_runtime::SudoConfig { key: endowed_accounts[0].clone() },
		balances: selendra_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		parachain_info: selendra_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: selendra_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: selendra_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		democracy: selendra_runtime::DemocracyConfig::default(),
		scheduler: selendra_runtime::SchedulerConfig {},
		council_collective: selendra_runtime::CouncilCollectiveConfig {
			phantom: Default::default(),
			members: endowed_accounts.clone(),
		},
		tech_committee_collective: selendra_runtime::TechCommitteeCollectiveConfig {
			phantom: Default::default(),
			members: endowed_accounts.clone(),
		},
		treasury: Default::default(),
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
