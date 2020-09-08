use serde_json::json;
use std::convert::TryInto;

use hex_literal::hex;

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use jupiter_runtime::{AccountId, SessionKeys, Signature};
use jupiter_runtime::{
    AuraConfig, BalancesConfig, ContractsConfig, GenesisConfig, GrandpaConfig, IndicesConfig,
    SessionConfig, SudoConfig, SystemConfig, WASM_BINARY,
};

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

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
type AuthorityKeysTuple = (
    AccountId, // ValidatorId
    AccountId, // (SessionKey)
    AuraId,
    GrandpaId,
);
/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> AuthorityKeysTuple {
    (
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_from_seed::<AuraId>(seed),
        get_from_seed::<GrandpaId>(seed),
    )
}
fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
    SessionKeys { aura, grandpa }
}

pub fn development_config() -> Result<ChainSpec, String> {
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
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
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

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
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
                true,
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

pub fn staging_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Testnet wasm binary not available".to_string())?;

    let root_key: AccountId =
        hex!["8a5b214c82362a8aba60af2a5fee63989f1ae4ce3ec802251c0b3ff9f4ad1826"].into();
    // 1
    let (stash_1, controller_1): (AccountId, AccountId) = (
        // 5FbxS7sovNuk5DddziwwQL9MzsNbidQnf5NEuqzNWcPj1pW4
        hex!["9c8a60a79c1a5fe62597915c56659be3b390f8d52a0ee573a92d8dc55a460a4a"].into(),
        // 5F4KwD8s2y2GNRjKUrZNw5eExocPaVm1Nsf5rTvouZYrRVJN
        hex!["846ae7c3e25ea7762db990854800a24a10a5255e7de6c467a277ed288a1e9577"].into(),
    );
    let (aura_1, grandpa_1): (AuraId, GrandpaId) = (
        // 5EtZp8WCs4yRDvfLapwU8GrSoS9e7qpXVtYZVCNDFBXuVsiW
        hex!["7cf92b27e280cef89900a7d351e37cdc6a578104a71f918165f52fee77aef647"].unchecked_into(),
        // 5EFhVePXcH3bQ8z4QQjqeBQGi6QqJyR4NpEYSpVjV59gwy6w
        hex!["60daf23817b1b4f247c08ed3b93480dd45c021e7ddeb95ecff74a09fe6acec0a"].unchecked_into(),
    );
    // 2
    let (stash_2, controller_2): (AccountId, AccountId) = (
        // 5CVTFqhjfPfMWHrKXJAZNb3qVahz32pYKQww2o5rFFCgHg2Y
        hex!["12dffc4900a3dc57d9ffb464f8a6af677839329b7b6c7d6a514db1ce55d17c12"].into(),
        // 5DMRmiSn4DTdo7HZM2frH88UZFapYGMCgEQukpD6yd5fc1Pm
        hex!["38fd41ccf5b2020e4f79ab9007e0496096ff5edd692c57cba1dbfce5eaa77c69"].into(),
    );
    let (aura_2, grandpa_2): (AuraId, GrandpaId) = (
        // 5GRaVtCnSKjeXXXu6S5cVEcqn5yGbnCTGYYeKodxiRbnnZBd
        hex!["c0dc511c88d7ef3bc5b465de48aac4efc643283f179b93d413f97e4a7aac714b"].unchecked_into(),
        // 5ESFwnQnTTFPgtBNxDWoPwrSvPSz9TYUndePjQ7ct2Cc4XdS
        hex!["68e8a38d283644652f0e8848a3db718f62bf8bccd0cbaa5b0dfeb8073c101a6a"].unchecked_into(),
    );

    let initial_authorities: Vec<AuthorityKeysTuple> = vec![
        (controller_1.clone(), stash_1.clone(), aura_1, grandpa_1),
        (controller_2.clone(), stash_2.clone(), aura_2, grandpa_2),
    ];

    Ok(ChainSpec::from_genesis(
        // Name
        "Jupiter Staging Testnet",
        // ID
        "jupiter_staging_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                initial_authorities.clone(),
                // Sudo account
                root_key.clone(),
                // Pre-funded accounts
                vec![
                    root_key.clone(),
                    controller_1.clone(),
                    stash_1.clone(),
                    controller_2.clone(),
                    stash_2.clone(),
                ],
                true,
            )
        },
        // Bootnodes
        vec![
            "/dns/p2p.staging-1.patract.io/tcp/30333/p2p/12D3KooWERffViUKxaPU1XEpQ11wPyjBM775XZU6jkApz25wxzEZ".to_string().try_into().unwrap(),
            "/dns/p2p.staging-2.patract.io/tcp/30333/p2p/12D3KooWK4YK8jbwirhMsmfMX4QgyihZJGQz72G1Jd4iV8zLjjpq".to_string().try_into().unwrap(),
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
    initial_authorities: Vec<AuthorityKeysTuple>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    enable_println: bool,
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
        pallet_aura: Some(AuraConfig {
            authorities: vec![],
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
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
    }
}
