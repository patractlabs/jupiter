use serde_json::json;
use serde::{Deserialize, Serialize};
use hex_literal::hex;

use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use jupiter_para_runtime::{AccountId, Signature};
use jupiter_para_runtime::{
    BalancesConfig, ContractsConfig, GenesisConfig, IndicesConfig,
    SudoConfig, SystemConfig, ParachainInfoConfig
};
use cumulus_primitives::ParaId;
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

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config(id: ParaId) -> Result<ChainSpec, String> {
    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                true,
                id,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(
            json!({
                "ss58Format": 42,
                "tokenDecimals": 10,
                "tokenSymbol": "J-DOT"
            })
            .as_object()
            .expect("network properties generation can not fail; qed")
            .to_owned(),
        ),
        // Extensions
        Extensions {
            relay_chain: "rococo".into(),
            para_id: id.into(),
        },
    ))
}

pub fn staging_testnet_config(id: ParaId) -> Result<ChainSpec, String> {
    Ok(ChainSpec::from_genesis(
        "Jupiter Parachain Staging Testnet",
        "jupiter_para staging_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                hex!["426d8def6146e8ae997b24f81401e46e8439d7f392489549b10410bcca20b64e"].into(),
                vec![
                    hex!["426d8def6146e8ae997b24f81401e46e8439d7f392489549b10410bcca20b64e"].into(),
                ],
                false,
                id,
            )
        },
        Vec::new(),
        Some(TelemetryEndpoints::new(
            vec![
                (PATRACT_TELEMETRY_URL.to_string(), 0)
            ])
                 .expect("Polkadot Staging telemetry url is valid; qed"),
        ),
        Some("jupiter_para_staging_testnet"),
        Some(
            json!({
                "ss58Format": 42,
                "tokenDecimals": 10,
                "tokenSymbol": "J-DOT"
            })
                .as_object()
                .expect("network properties generation can not fail; qed")
                .to_owned(),
        ),
        Extensions {
            relay_chain: "rococo".into(),
            para_id: id.into(),
        },
    ))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/testnet.json")[..])
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    enable_println: bool,
    id: ParaId,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: jupiter_para_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println, // this should only be enabled on development chains
                ..Default::default()
            },
        }),
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key,
        }),
        parachain_info: Some(ParachainInfoConfig { parachain_id: id }),
    }
}
