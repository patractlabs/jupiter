use serde_json::json;
use std::convert::TryInto;

use hex_literal::hex;

use sc_service::{config::TelemetryEndpoints, ChainType};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};

use jupiter_runtime::{AccountId, SessionKeys, Signature};
use jupiter_runtime::{
    AuthorityDiscoveryConfig, BalancesConfig, ContractsConfig, CouncilConfig, GenesisConfig,
    IndicesConfig, SessionConfig, StakingConfig, SudoConfig, SystemConfig,
    TechnicalCommitteeConfig, WASM_BINARY,
};
use jupiter_runtime_common::constants::currency::DOTS;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::{Forcing, StakerStatus};

// The URL for the telemetry server.
const PATRACT_TELEMETRY_URL: &str = "wss://telemetry.patract.io/submit";

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

/// Generate an Babe authority key.
type AuthorityKeysTuple = (
    AccountId, // ValidatorId
    AccountId, // (SessionKey)
    BabeId,
    GrandpaId,
    ImOnlineId,
    AuthorityDiscoveryId,
);
/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> AuthorityKeysTuple {
    (
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

fn session_keys(
    babe: BabeId,
    grandpa: GrandpaId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        babe,
        grandpa,
        im_online,
        authority_discovery,
    }
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
                "tokenSymbol": "JPT"
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
                "tokenSymbol": "JPT"
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

    // subkey inspect "$SECRET"
    // 5FC7Y5AHZgsGsdPpkZzhNNMVEvwkV1JDBhFgoDRifbnfFQhp
    let root_key: AccountId =
        hex!["8a5b214c82362a8aba60af2a5fee63989f1ae4ce3ec802251c0b3ff9f4ad1826"].into();
    // bash:
    // for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
    // for i in 1 2 3 4; do for j in babe; do subkey inspect --scheme Sr25519 "$SECRET//$i//$j"; done; done
    // for i in 1 2 3 4; do for j in grandpa; do subkey inspect --scheme Ed25519 "$SECRET//$i//$j"; done; done
    // for i in 1 2 3 4; do for j in im_online; do subkey inspect --scheme Sr25519 "$SECRET//$i//$j"; done; done

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
    let (stash3, controller3): (AccountId, AccountId) = (
        // 5EEbdKijxzqspBP1SKAbE7gREHx4m5AXfAAB9vaga9rWkEFB
        hex!["6003f2d767ffb437bad9a5cdf0ea3782cbb36207edd3e284a84eba2419ae3c4d"].into(),
        // 5G9ALoW7Ccpm8zDwz21RdGHRFhfGYQKFgMDf7JkYXFCnLKHo
        hex!["b4571275b5dfdcafca6f42e77dd627da0ecde9bd9e60426b38d90dbca511a654"].into(),
    );
    let (stash4, controller4): (AccountId, AccountId) = (
        // 5GvFNec4YCaH2Wu5ysmG938AjtHcrka2h6u8GD9aEiJHut7n
        hex!["d6ba0dd1b58e3fe1a380f51f289e5745d2bdf3dfef712e5febaea4f867c57429"].into(),
        // 5Cafs4eRGLi9yyat5YwduLNL2xZA36PoLKjvApk1Cm1ScMwA
        hex!["16daa7070bf20d6170f39b468b5abadd5d0ad810f7c1e8303fa48a79081d6d44"].into(),
    );
    // babe
    // 5DhZaXAow1FwaJPSn2JNHxMqsmjp6reNLWQmPNuqq5PNZ4CW
    let babe1: BabeId =
        hex!["485872201949c11fed2706383b0bdece75f891e579bf44458b7e40d90ab7b84d"].unchecked_into();
    // 5H3wddAi5xfRvGApjJ4ZQj3bwHf2UNxRGdy6397SxZ48ryyK
    let babe2: BabeId =
        hex!["dc984b002d46efcadf477db5fa64fccc14282999663ec909bb6e90b16de24e23"].unchecked_into();
    // 5Do1StjFe2SPe6fUCKWyVwTg9CKGAd2ZRir2e7XSdQVo6K83
    let babe3: BabeId =
        hex!["4c7fc0fe786089da8571fb20b2546d31b6be583b4c640cdf2878461c6ab6ee15"].unchecked_into();
    // 5Gs4SpDKNQDZxPtB3asd6B3M7mwo8VMasW7YLwLiNu1or4cG
    let babe4: BabeId =
        hex!["d44b87bdd403d8b7380ae2380cae6fea08a965c8910791c89b8b595eac90e474"].unchecked_into();
    // grandpa
    // 5CwDbbQZY7dWGhgPvGqbi2WnhcFw7WkpW2ZrmvSvhtM7qVKn
    let grandpa1: GrandpaId =
        hex!["26866260bb8dabbe84fe7d2fee7bed533104a8e890fefc0001813bbd0e5c9cb9"].unchecked_into();
    // 5G6VvSAqrqNLoqoRM4GUxrKQkfHuZ3ggnW8bbsQZbi187kfu
    let grandpa2: GrandpaId =
        hex!["b24f3f629e5cc692b50f08b9714c67be656b057293bb077beb44781a9a0e2992"].unchecked_into();
    // 5DTqERGezEdBkhNKaAczspP9MTzv73GAQr91Rf1t82csFUbJ
    let grandpa3: GrandpaId =
        hex!["3ddfb449782bb812d1a85a1a05870667549847c254ab981a4586c43d89f09203"].unchecked_into();
    // 5FVKQ3FXyUSGWt8eBK6Q2NJuzoZaSBtoukGPfqbMfb4RtqV4
    let grandpa4: GrandpaId =
        hex!["977a3b37d4be87bcdc027461c30d6c5f7a0e2c4fc11c91b618444a1be9a2b18c"].unchecked_into();
    // im_online
    // 5DCVpVpaogBmFaovbpLkvjdzwsNoASYZoFDhyLvGBCi4YdLa
    let imonline1: ImOnlineId =
        hex!["322dacd21c5f1db0a530fb8e3d87c5347d9c2e01aab574fea5797ef76a239147"].unchecked_into();
    // 5EcYTkD6yYBUvtfBi7Y5VtZzCDMfqhGbe2LWTm7yrjudC8VC
    let imonline2: ImOnlineId =
        hex!["70c0b15510e3d2d1fda9dbc7568c211932ef6aa49eb4bf48a7068dbeb9acf26e"].unchecked_into();
    // 5EWsJmZebbHLAnghieEMKrpBpTmu7bSGaT6VRfcV7tpmFnvX
    let imonline3: ImOnlineId =
        hex!["6c6ca8b2c42dd0f37981a89998ef37ccb05b6808518e96e01a3d920e02cb6452"].unchecked_into();
    // 5ESAnfDPdLXAozgazozAPsyoJXy6xD9c1WBbyKQ4pxRZGn5D
    let imonline4: ImOnlineId =
        hex!["68d7471ed1508b29497258a46a0cd0bd2eb5b3cce8eefb03b09044827b3bdf0c"].unchecked_into();
    // authority_discovery (same as im_online)
    // 5DCVpVpaogBmFaovbpLkvjdzwsNoASYZoFDhyLvGBCi4YdLa
    let audi1: AuthorityDiscoveryId =
        hex!["322dacd21c5f1db0a530fb8e3d87c5347d9c2e01aab574fea5797ef76a239147"].unchecked_into();
    // 5EcYTkD6yYBUvtfBi7Y5VtZzCDMfqhGbe2LWTm7yrjudC8VC
    let audi2: AuthorityDiscoveryId =
        hex!["70c0b15510e3d2d1fda9dbc7568c211932ef6aa49eb4bf48a7068dbeb9acf26e"].unchecked_into();
    // 5EWsJmZebbHLAnghieEMKrpBpTmu7bSGaT6VRfcV7tpmFnvX
    let audi3: AuthorityDiscoveryId =
        hex!["6c6ca8b2c42dd0f37981a89998ef37ccb05b6808518e96e01a3d920e02cb6452"].unchecked_into();
    // 5ESAnfDPdLXAozgazozAPsyoJXy6xD9c1WBbyKQ4pxRZGn5D
    let audi4: AuthorityDiscoveryId =
        hex!["68d7471ed1508b29497258a46a0cd0bd2eb5b3cce8eefb03b09044827b3bdf0c"].unchecked_into();

    let initial_authorities: Vec<AuthorityKeysTuple> = vec![
        (
            controller1.clone(),
            stash1.clone(),
            babe1,
            grandpa1,
            imonline1,
            audi1,
        ),
        (
            controller2.clone(),
            stash2.clone(),
            babe2,
            grandpa2,
            imonline2,
            audi2,
        ),
        (
            controller3.clone(),
            stash3.clone(),
            babe3,
            grandpa3,
            imonline3,
            audi3,
        ),
        (
            controller4.clone(),
            stash4.clone(),
            babe4,
            grandpa4,
            imonline4,
            audi4,
        ),
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
                    controller1.clone(),
                    stash1.clone(),
                    controller2.clone(),
                    stash2.clone(),
                    controller3.clone(),
                    stash3.clone(),
                    controller4.clone(),
                    stash4.clone(),
                ],
                true,
            )
        },
        // Bootnodes
        vec![
            "/dns/staging-1.jupiter.p2p.patract.io/tcp/30333/p2p/12D3KooWA8LCSwvSvrsVL5ANMzY5z23NDKrnSqLtyNq3geRmUsRT".to_string().try_into().expect("must be valid bootnode"),
            "/dns/staging-2.jupiter.p2p.patract.io/tcp/30334/p2p/12D3KooWSZAzpj81ouT2bZJFbfL6JTKiMiGZZq82AJZMUDB23JJ5".to_string().try_into().expect("must be valid bootnode"),
            "/dns/staging-3.jupiter.p2p.patract.io/tcp/30335/p2p/12D3KooWEm8wPo1AAZeiNifEGPz6doJeQeyk3z5343scxpiU46ef".to_string().try_into().expect("must be valid bootnode"),
            "/dns/staging-4.jupiter.p2p.patract.io/tcp/30336/p2p/12D3KooWMx9TH6AtZf8y7DnVxDfYJmLPrL43uVbrhsnH8JtsvH2r".to_string().try_into().expect("must be valid bootnode"),
        ],
        // Telemetry
        Some(TelemetryEndpoints::new(
            vec![
                (PATRACT_TELEMETRY_URL.to_string(), 0)
            ])
                 .expect("Polkadot Staging telemetry url is valid; qed"),
        ),
        // Protocol ID
        Some("jupiter_staging_testnet"),
        // Properties
        Some(
            json!({
                "ss58Format": 42,
                "tokenDecimals": 10,
                "tokenSymbol": "JPT"
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
    const ENDOWMENT: u128 = 1_000_000 * DOTS;
    const STASH: u128 = 100 * DOTS;

    let mut endowed_accounts: Vec<AccountId> = endowed_accounts;
    initial_authorities.iter().for_each(|x| {
        if !endowed_accounts.contains(&x.0) {
            endowed_accounts.push(x.0.clone())
        }
    });

    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .map(|k: &AccountId| (k.clone(), ENDOWMENT))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
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
        pallet_staking: Some(StakingConfig {
            validator_count: if initial_authorities.len() < 4 {
                initial_authorities.len() as u32 * 2
            } else {
                50
            },
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: Forcing::ForceNone,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_elections_phragmen: Some(Default::default()),
        pallet_democracy: Some(Default::default()),
        pallet_collective_Instance1: Some(CouncilConfig {
            members: vec![],
            phantom: Default::default(),
        }),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: vec![],
            phantom: Default::default(),
        }),
        pallet_membership_Instance1: Some(Default::default()),
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key,
        }),
    }
}
