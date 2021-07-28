use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_json::json;

use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify, Zero};

use cumulus_primitives_core::ParaId;
use jupiter_primitives::{AccountId, Signature};
use jupiter_runtime::{
    AuraId,
    BalancesConfig,
    GenesisConfig,
    ParachainInfoConfig,
    SudoConfig,
    SystemConfig, //ContractsConfig,
};
use sc_service::config::TelemetryEndpoints;

// The URL for the telemetry server.
const PATRACT_TELEMETRY_URL: &str = "wss://telemetry.patract.io/submit";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecExtension)]
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

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

pub fn session_keys(keys: AuraId) -> jupiter_runtime::opaque::SessionKeys {
    jupiter_runtime::opaque::SessionKeys { aura: keys }
}

/// Jupiter Development Chain Config
pub fn development_config(id: ParaId, relay_chain: &str) -> Result<ChainSpec, String> {
    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "jupiter_dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
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
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                id,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("jupiter_dev"),
        // Properties
        Some(
            json!({
                "ss58Format": jupiter_runtime::SS58Prefix::get(),
                "tokenDecimals": 10,
                "tokenSymbol": "DOT"
            })
            .as_object()
            .expect("network properties generation can not fail; qed")
            .to_owned(),
        ),
        // Extensions
        Extensions {
            relay_chain: relay_chain.into(),
            para_id: id.into(),
        },
    ))
}

/// Jupiter PC1 Staging Chain Config
pub fn staging_config(id: ParaId) -> Result<ChainSpec, String> {
    Ok(ChainSpec::from_genesis(
        "Jupiter PC1 Staging",
        "jupiter_pc1_staging",
        ChainType::Live,
        move || {
            testnet_genesis(
                // subkey inspect "$SECRET"
                hex!["426d8def6146e8ae997b24f81401e46e8439d7f392489549b10410bcca20b64e"].into(),
                // for i in 1 2; do subkey inspect "$SECRET//$i"
                // for i in 1 2; do for j in aura; do subkey inspect --scheme Sr25519 "$SECRET//$i//$j"; done; done
                vec![
                    (
                        hex!["b4ae25a975444e139b44bfe7fcdc4e8389c29687492b6c6958f4250e989cc877"]
                            .into(),
                        hex!["0a7d580f81d12479b6fd1e27af50cd67a1c95bfee8b9527d56915363d97f4618"]
                            .unchecked_into(),
                    ),
                    (
                        hex!["964974e3fb8dd02a34baa3381a7a7242e99a5677e807aab940437a22912e0f6b"]
                            .into(),
                        hex!["e8780f81d1448511c7fedc40284070042bfc422f7863d1a8291fb15e4ef0f72f"]
                            .unchecked_into(),
                    ),
                ],
                vec![
                    hex!["426d8def6146e8ae997b24f81401e46e8439d7f392489549b10410bcca20b64e"].into(),
                ],
                id,
            )
        },
        Vec::new(),
        Some(
            TelemetryEndpoints::new(vec![(PATRACT_TELEMETRY_URL.to_string(), 0)])
                .expect("Polkadot Staging telemetry url is valid; qed"),
        ),
        Some("jupiter_pc1_staging"),
        Some(
            json!({
                "ss58Format": jupiter_runtime::SS58Prefix::get(),
                "tokenDecimals": 12,
                "tokenSymbol": "WND"
            })
            .as_object()
            .expect("network properties generation can not fail; qed")
            .to_owned(),
        ),
        Extensions {
            relay_chain: "westend-staging".into(),
            para_id: id.into(),
        },
    ))
}

/// Jupiter Chain Config
pub fn jupiter_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../res/jupiter-westend-patract.json")[..])
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    root_key: AccountId,
    initial_authorities: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: jupiter_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 100))
                .collect(),
        },
        // indices: IndicesConfig { indices: vec![] },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
        parachain_info: ParachainInfoConfig { parachain_id: id },
        collator_selection: jupiter_runtime::CollatorSelectionConfig {
            invulnerables: initial_authorities
                .iter()
                .cloned()
                .map(|(acc, _)| acc)
                .collect(),
            candidacy_bond: Zero::zero(),
            ..Default::default()
        },
        session: jupiter_runtime::SessionConfig {
            keys: initial_authorities
                .iter()
                .cloned()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),        // account id
                        acc.clone(),        // validator id
                        session_keys(aura), // session keys
                    )
                })
                .collect(),
        },
        aura: Default::default(),
        // aura: jupiter_runtime::AuraConfig {
        //     authorities: initial_authorities,
        // },
        aura_ext: Default::default(),
        parachain_system: Default::default(),
    }
}
