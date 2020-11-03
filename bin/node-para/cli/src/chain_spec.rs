use serde_json::json;
use std::convert::TryInto;
use serde::{Deserialize, Serialize};

use hex_literal::hex;

use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use jupiter_para_runtime::{AccountId, Signature};
use jupiter_para_runtime::{
    BalancesConfig, ContractsConfig, GenesisConfig, IndicesConfig,
    SudoConfig, SystemConfig, WASM_BINARY, ParachainInfoConfig
};
use cumulus_primitives::ParaId;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

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
    pub fn try_get(chain_spec: &Box<dyn sc_service::ChainSpec>) -> Option<&Self> {
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
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
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
                "tokenSymbol": "DOT (new)"
            })
            .as_object()
            .expect("network properties generation can not fail; qed")
            .to_owned(),
        ),
        // Extensions
        None,
    ))
}

pub fn staging_testnet_config(id: ParaId) -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Testnet wasm binary not available".to_string())?;

    // subkey inspect-key --uri "$SECRET"
    // 5FC7Y5AHZgsGsdPpkZzhNNMVEvwkV1JDBhFgoDRifbnfFQhp
    let root_key: AccountId =
        hex!["8a5b214c82362a8aba60af2a5fee63989f1ae4ce3ec802251c0b3ff9f4ad1826"].into();
    // bash:
    // for i in 1 2; do for j in stash controller; do subkey inspect-key --uri "$SECRET//$i//$j"; done; done
    // for i in 1 2; do for j in aura; do subkey inspect-key --scheme sr25519 --uri "$SECRET//$i//$j"; done; done
    // for i in 1 2; do for j in grandpa; do subkey inspect-key --scheme ed25519 --uri "$SECRET//$i//$j"; done; done

    // stash & controller
    let (stash1, controller1): (AccountId, AccountId) = (
        // 5FbxS7sovNuk5DddziwwQL9MzsNbidQnf5NEuqzNWcPj1pW4
        hex!["9c8a60a79c1a5fe62597915c56659be3b390f8d52a0ee573a92d8dc55a460a4a"].into(),
        // 5F4KwD8s2y2GNRjKUrZNw5eExocPaVm1Nsf5rTvouZYrRVJN
        hex!["846ae7c3e25ea7762db990854800a24a10a5255e7de6c467a277ed288a1e9577"].into(),
    );
    let (stash2, controller2): (AccountId, AccountId) = (
        // 5CVTFqhjfPfMWHrKXJAZNb3qVahz32pYKQww2o5rFFCgHg2Y
        hex!["12dffc4900a3dc57d9ffb464f8a6af677839329b7b6c7d6a514db1ce55d17c12"].into(),
        // 5DMRmiSn4DTdo7HZM2frH88UZFapYGMCgEQukpD6yd5fc1Pm
        hex!["38fd41ccf5b2020e4f79ab9007e0496096ff5edd692c57cba1dbfce5eaa77c69"].into(),
    );

    Ok(ChainSpec::from_genesis(
        // Name
        "Jupiter Staging Testnet",
        // ID
        "jupiter_staging_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Sudo account
                root_key.clone(),
                // Pre-funded accounts
                vec![
                    root_key.clone(),
                    controller1.clone(),
                    stash1.clone(),
                    controller2.clone(),
                    stash2.clone(),
                ],
                true,
                id,
            )
        },
        // Bootnodes
        vec![
            "/dns/p2p.staging-1.patract.io/tcp/30333/p2p/12D3KooWERffViUKxaPU1XEpQ11wPyjBM775XZU6jkApz25wxzEZ".to_string().try_into().expect("must be valid bootnode"),
            "/dns/p2p.staging-2.patract.io/tcp/30334/p2p/12D3KooWK4YK8jbwirhMsmfMX4QgyihZJGQz72G1Jd4iV8zLjjpq".to_string().try_into().expect("must be valid bootnode"),
        ],
        // Telemetry
        None,
        // Protocol ID
        Some("jupiter_staging_testnet"),
        // Properties
        Some(
            json!({
                "ss58Format": 42,
                "tokenDecimals": 10,
                "tokenSymbol": "DOT (new)"
            })
            .as_object()
            .expect("network properties generation can not fail; qed")
            .to_owned(),
        ),
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    enable_println: bool,
    id: ParaId,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
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
