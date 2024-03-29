//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::sync::Arc;
use std::time::Duration;

use futures::prelude::*;
use jupiter_executor::Executor;
use jupiter_primitives::Block;
use sc_client_api::ExecutorProvider;
use sc_consensus_babe::SlotProportion;
use sc_executor::NativeElseWasmExecutor;
use sc_finality_grandpa::FinalityProofProvider as GrandpaFinalityProofProvider;
use sc_network::{Event, NetworkService};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_runtime::traits::Block as BlockT;

use jupiter_runtime::{self, RuntimeApi};

type FullClient = sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport =
    sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;

pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        sc_consensus::DefaultImportQueue<Block, FullClient>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (
            (
                sc_consensus_babe::BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
                sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
                sc_consensus_babe::BabeLink<Block>,
            ),
            sc_finality_grandpa::SharedVoterState,
            Option<Telemetry>,
        ),
    >,
    ServiceError,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = NativeElseWasmExecutor::<Executor>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>(
            &config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;

    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>),
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    let (block_import, babe_link) = sc_consensus_babe::block_import(
        sc_consensus_babe::Config::get_or_compute(&*client)?,
        grandpa_block_import.clone(),
        client.clone(),
    )?;

    let slot_duration = babe_link.config().slot_duration();
    let import_queue = sc_consensus_babe::import_queue(
        babe_link.clone(),
        block_import.clone(),
        Some(Box::new(grandpa_block_import)),
        client.clone(),
        select_chain.clone(),
        move |_, ()| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

            let slot =
                sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
                    *timestamp,
                    slot_duration,
                );

            let uncles =
                sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

            Ok((timestamp, slot, uncles))
        },
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
        sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    let import_setup = (block_import, grandpa_link, babe_link);
    let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();
    let rpc_setup = shared_voter_state.clone();

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        keystore_container,
        select_chain,
        import_queue,
        transaction_pool,
        other: (import_setup, rpc_setup, telemetry),
    })
}

pub struct NewFullBase {
    pub task_manager: TaskManager,
    pub client: Arc<FullClient>,
    pub network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
    pub transaction_pool: Arc<sc_transaction_pool::FullPool<Block, FullClient>>,
}

/// Creates a full service from the configuration.
pub fn new_full_base(mut config: Configuration) -> Result<NewFullBase, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (import_setup, rpc_setup, mut telemetry),
    } = new_partial(&config)?;

    let shared_voter_state = rpc_setup;

    config
        .network
        .extra_sets
        .push(sc_finality_grandpa::grandpa_peers_set_config());

    let (block_import, grandpa_link, babe_link) = import_setup;

    let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
        backend.clone(),
        grandpa_link.shared_authority_set().clone(),
        Vec::default(),
    ));

    let (network, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync: Some(warp_sync),
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let role = config.role.clone();
    let force_authoring = config.force_authoring;
    let backoff_authoring_blocks =
        Some(sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging::default());
    let name = config.network.node_name.clone();
    let enable_grandpa = !config.disable_grandpa;
    let prometheus_registry = config.prometheus_registry().cloned();

    let rpc_extensions_builder = {
        let justification_stream = grandpa_link.justification_stream();
        let shared_authority_set = grandpa_link.shared_authority_set().clone();
        let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();

        let finality_proof_provider = GrandpaFinalityProofProvider::new_for_service(
            backend.clone(),
            Some(shared_authority_set.clone()),
        );

        let babe_config = babe_link.config().clone();
        let shared_epoch_changes = babe_link.epoch_changes().clone();

        let client = client.clone();
        let pool = transaction_pool.clone();
        let select_chain = select_chain.clone();
        let keystore = keystore_container.sync_keystore();

        Box::new(move |deny_unsafe, subscription_executor| {
            let deps = jupiter_rpc::FullDeps {
                basic: jupiter_rpc::BasicDeps {
                    client: client.clone(),
                    pool: pool.clone(),
                    deny_unsafe,
                },
                select_chain: select_chain.clone(),
                babe: jupiter_rpc::BabeDeps {
                    babe_config: babe_config.clone(),
                    shared_epoch_changes: shared_epoch_changes.clone(),
                    keystore: keystore.clone(),
                },
                grandpa: jupiter_rpc::GrandpaDeps {
                    shared_voter_state: shared_voter_state.clone(),
                    shared_authority_set: shared_authority_set.clone(),
                    justification_stream: justification_stream.clone(),
                    subscription_executor,
                    finality_provider: finality_proof_provider.clone(),
                },
            };

            Ok(jupiter_rpc::create_full(deps))
        })
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        config,
        backend: backend.clone(),
        client: client.clone(),
        keystore: keystore_container.sync_keystore(),
        network: network.clone(),
        rpc_extensions_builder,
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        system_rpc_tx,
        telemetry: telemetry.as_mut(),
    })?;

    if let sc_service::config::Role::Authority { .. } = &role {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        let can_author_with =
            sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

        let client_clone = client.clone();
        let slot_duration = babe_link.config().slot_duration();
        let babe_config = sc_consensus_babe::BabeParams {
            keystore: keystore_container.sync_keystore(),
            client: client.clone(),
            select_chain,
            env: proposer,
            block_import,
            sync_oracle: network.clone(),
            justification_sync_link: network.clone(),
            create_inherent_data_providers: move |parent, ()| {
                let client_clone = client_clone.clone();
                async move {
                    let uncles = sc_consensus_uncles::create_uncles_inherent_data_provider(
                        &*client_clone,
                        parent,
                    )?;

                    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                    let slot =
                        sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
                            *timestamp,
                            slot_duration,
                        );

                    Ok((timestamp, slot, uncles))
                }
            },
            force_authoring,
            backoff_authoring_blocks,
            babe_link,
            can_author_with,
            block_proposal_slot_portion: SlotProportion::new(0.5),
            max_block_proposal_slot_portion: None,
            telemetry: telemetry.as_ref().map(|x| x.handle()),
        };

        let babe = sc_consensus_babe::start_babe(babe_config)?;
        task_manager
            .spawn_essential_handle()
            .spawn_blocking("babe-proposer", None, babe);
    }

    // Spawn authority discovery module.
    if role.is_authority() {
        let authority_discovery_role =
            sc_authority_discovery::Role::PublishAndDiscover(keystore_container.keystore());
        let dht_event_stream =
            network
                .event_stream("authority-discovery")
                .filter_map(|e| async move {
                    match e {
                        Event::Dht(e) => Some(e),
                        _ => None,
                    }
                });
        let (authority_discovery_worker, _service) = sc_authority_discovery::new_worker_and_service(
            client.clone(),
            network.clone(),
            Box::pin(dht_event_stream),
            authority_discovery_role,
            prometheus_registry.clone(),
        );

        task_manager.spawn_handle().spawn(
            "authority-discovery-worker",
            None,
            authority_discovery_worker.run(),
        );
    }

    // if the node isn't actively participating in consensus then it doesn't
    // need a keystore, regardless of which protocol we use below.
    let keystore = if role.is_authority() {
        Some(keystore_container.sync_keystore())
    } else {
        None
    };

    let grandpa_config = sc_finality_grandpa::Config {
        // FIXME #1578 make this available through chainspec
        gossip_duration: Duration::from_millis(333),
        justification_period: 512,
        name: Some(name),
        observer_enabled: false,
        keystore,
        local_role: role,
        telemetry: telemetry.as_ref().map(|x| x.handle()),
    };

    if enable_grandpa {
        // start the full GRANDPA voter
        // NOTE: non-authorities could run the GRANDPA observer protocol, but at
        // this point the full voter should provide better guarantees of block
        // and vote data availability than the observer. The observer has not
        // been tested extensively yet and having most nodes in a network run it
        // could lead to finality stalls.
        let grandpa_config = sc_finality_grandpa::GrandpaParams {
            config: grandpa_config,
            link: grandpa_link,
            network: network.clone(),
            telemetry: telemetry.as_ref().map(|x| x.handle()),
            voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
            prometheus_registry,
            shared_voter_state,
        };

        // the GRANDPA voter task is considered infallible, i.e.
        // if it fails we take down the service with it.
        task_manager.spawn_essential_handle().spawn_blocking(
            "grandpa-voter",
            None,
            sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
        );
    }

    network_starter.start_network();
    Ok(NewFullBase {
        task_manager,
        client,
        network,
        transaction_pool,
    })
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    new_full_base(config).map(|NewFullBase { task_manager, .. }| task_manager)
}

// Builds a new service for a light client.
// pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
//     let telemetry = config
//         .telemetry_endpoints
//         .clone()
//         .filter(|x| !x.is_empty())
//         .map(|endpoints| -> Result<_, sc_telemetry::Error> {
//             #[cfg(feature = "browser")]
//             let transport = Some(sc_telemetry::ExtTransport::new(
//                 libp2p_wasm_ext::ffi::websocket_transport(),
//             ));
//             #[cfg(not(feature = "browser"))]
//             let transport = None;

//             let worker = TelemetryWorker::with_transport(16, transport)?;
//             let telemetry = worker.handle().new_telemetry(endpoints);
//             Ok((worker, telemetry))
//         })
//         .transpose()?;

//     let (client, backend, keystore_container, mut task_manager, on_demand) =
//         sc_service::new_light_parts::<Block, RuntimeApi, Executor>(
//             &config,
//             telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
//         )?;

//     let mut telemetry = telemetry.map(|(worker, telemetry)| {
//         task_manager.spawn_handle().spawn("telemetry", worker.run());
//         telemetry
//     });

//     let select_chain = sc_consensus::LongestChain::new(backend.clone());

//     let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
//         config.transaction_pool.clone(),
//         config.prometheus_registry(),
//         task_manager.spawn_essential_handle(),
//         client.clone(),
//         on_demand.clone(),
//     ));

//     let (grandpa_block_import, _) = sc_finality_grandpa::block_import(
//         client.clone(),
//         &(client.clone() as Arc<_>),
//         select_chain.clone(),
//         telemetry.as_ref().map(|x| x.handle()),
//     )?;

//     let (babe_block_import, babe_link) = sc_consensus_babe::block_import(
//         sc_consensus_babe::Config::get_or_compute(&*client)?,
//         grandpa_block_import.clone(),
//         client.clone(),
//     )?;

//     let slot_duration = babe_link.config().slot_duration();
//     let import_queue = sc_consensus_babe::import_queue(
//         babe_link,
//         babe_block_import,
//         Some(Box::new(grandpa_block_import)),
//         client.clone(),
//         select_chain.clone(),
//         move |_, ()| async move {
//             let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

//             let slot =
//                 sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
//                     *timestamp,
//                     slot_duration,
//                 );

//             let uncles =
//                 sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

//             Ok((timestamp, slot, uncles))
//         },
//         &task_manager.spawn_essential_handle(),
//         config.prometheus_registry(),
//         sp_consensus::NeverCanAuthor,
//         telemetry.as_ref().map(|x| x.handle()),
//     )?;

//     let (network, system_rpc_tx, network_starter) =
//         sc_service::build_network(sc_service::BuildNetworkParams {
//             config: &config,
//             client: client.clone(),
//             transaction_pool: transaction_pool.clone(),
//             spawn_handle: task_manager.spawn_handle(),
//             import_queue,
//             block_announce_validator_builder: None,
//             warp_sync: None,
//         })?;
//     network_starter.start_network();

//     if config.offchain_worker.enabled {
//         sc_service::build_offchain_workers(
//             &config,
//             task_manager.spawn_handle(),
//             client.clone(),
//             network.clone(),
//         );
//     }

//     let light_deps = jupiter_rpc::LightDeps {
//         remote_blockchain: backend.remote_blockchain(),
//         fetcher: on_demand.clone(),
//         client: client.clone(),
//         pool: transaction_pool.clone(),
//     };

//     let rpc_extensions = jupiter_rpc::create_light(light_deps);
//     sc_service::spawn_tasks(sc_service::SpawnTasksParams {
//         transaction_pool,
//         task_manager: &mut task_manager,
//         rpc_extensions_builder: Box::new(sc_service::NoopRpcExtensionBuilder(rpc_extensions)),
//         config,
//         client,
//         keystore: keystore_container.sync_keystore(),
//         backend,
//         network,
//         system_rpc_tx,
//         telemetry: telemetry.as_mut(),
//     })?;

//     Ok(task_manager)
// }
