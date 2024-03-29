//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::sync::Arc;

pub use sc_executor::NativeElseWasmExecutor;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};

use jupiter_primitives::Block;

use jupiter_dev_runtime::{self, RuntimeApi};

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
pub struct Executor;

impl sc_executor::NativeExecutionDispatch for Executor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        jupiter_dev_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        jupiter_dev_runtime::native_version()
    }
}

type FullClient = sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

/// Returns most parts of a service. Not enough to run a full chain,
/// But enough to perform chain operations like purge-chain
pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        sc_consensus::DefaultImportQueue<Block, FullClient>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (Option<Telemetry>,),
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

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let import_queue = sc_consensus_manual_seal::import_queue(
        Box::new(client.clone()),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    );

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        keystore_container,
        select_chain,
        import_queue,
        transaction_pool,
        other: (telemetry,),
    })
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    // This variable is only used when ocw feature is enabled.
    // Suppress the warning when ocw feature is not enabled.
    #[allow(unused_variables)]
    let dev_seed = config.dev_key_seed.clone();

    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (mut telemetry,),
    } = new_partial(&config)?;

    // Initialize seed for signing transaction using off-chain workers
    #[cfg(feature = "ocw")]
    {
        if let Some(seed) = dev_seed {
            service
                .keystore()
                .write()
                .insert_ephemeral_from_seed_by_type::<runtime::offchain_demo::crypto::Pair>(
                    &seed,
                    runtime::offchain_demo::KEY_TYPE,
                )
                .expect("Dev Seed should always succeed.");
        }
    }

    let (network, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync: None,
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
    let prometheus_registry = config.prometheus_registry().cloned();

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = jupiter_rpc::BasicDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
            };

            Ok(jupiter_rpc::create_basic(deps))
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        config,
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.sync_keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_extensions_builder,
        backend,
        // network_status_sinks,
        system_rpc_tx,
        telemetry: telemetry.as_mut(),
    })?;

    if role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        let authorship_future = sc_consensus_manual_seal::run_instant_seal(
            sc_consensus_manual_seal::InstantSealParams {
                block_import: client.clone(),
                env: proposer,
                client: client.clone(),
                pool: transaction_pool.clone(),
                select_chain,
                consensus_data_provider: None,
                create_inherent_data_providers: |_, _| async {
                    Ok((sp_timestamp::InherentDataProvider::from_system_time(), ()))
                },
            },
        );

        task_manager.spawn_essential_handle().spawn_blocking(
            "instant-seal",
            Some("block-authoring"),
            authorship_future,
        );
    };

    network_starter.start_network();
    Ok(task_manager)
}

// TODO: remove light client
// Builds a new service for a light client.
// pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
//     let telemetry = config
//         .telemetry_endpoints
//         .clone()
//         .filter(|x| !x.is_empty())
//         .map(|endpoints| -> Result<_, sc_telemetry::Error> {
//             let worker = TelemetryWorker::new(16)?;
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

//     let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
//         config.transaction_pool.clone(),
//         config.prometheus_registry(),
//         task_manager.spawn_essential_handle(),
//         client.clone(),
//         on_demand.clone(),
//     ));

//     let import_queue = sc_consensus_manual_seal::import_queue(
//         Box::new(client.clone()),
//         &task_manager.spawn_essential_handle(),
//         config.prometheus_registry(),
//     );

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

//     if config.offchain_worker.enabled {
//         sc_service::build_offchain_workers(
//             &config,
//             task_manager.spawn_handle(),
//             client.clone(),
//             network.clone(),
//         );
//     }

//     sc_service::spawn_tasks(sc_service::SpawnTasksParams {
//         transaction_pool,
//         task_manager: &mut task_manager,
//         rpc_extensions_builder: Box::new(|_, _| ()),
//         config,
//         client,
//         keystore: keystore_container.sync_keystore(),
//         backend,
//         network,
//         // network_status_sinks,
//         system_rpc_tx,
//         telemetry: telemetry.as_mut(),
//     })?;

//     network_starter.start_network();

//     Ok(task_manager)
// }
