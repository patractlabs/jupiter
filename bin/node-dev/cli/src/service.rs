//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::sync::Arc;

use sc_client_api::RemoteBackend;
use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_finality_grandpa::FinalityProofProvider as GrandpaFinalityProofProvider;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sp_inherents::InherentDataProviders;

use jupiter_primitives::Block;
use jupiter_runtime::{self, RuntimeApi};

use sc_network::config::DummyFinalityProofRequestBuilder;

// Our native executor instance.
native_executor_instance!(
    pub Executor,
    jupiter_runtime::api::dispatch,
    jupiter_runtime::native_version,
);

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
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
        sp_consensus::DefaultImportQueue<Block, FullClient>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (),
    >,
    ServiceError,
> {
    let inherent_data_providers = InherentDataProviders::new();
    inherent_data_providers
        .register_provider(sp_timestamp::InherentDataProvider)
        .map_err(Into::into)
        .map_err(sp_consensus::error::Error::InherentData)?;

    let (client, backend, keystore, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
    let client = Arc::new(client);

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
    );

    let import_queue = sc_consensus_manual_seal::import_queue(
        Box::new(client.clone()),
        &task_manager.spawn_handle(),
        config.prometheus_registry(),
    );

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore,
        select_chain,
        transaction_pool,
        inherent_data_providers,
        other: (),
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
        keystore,
        select_chain,
        transaction_pool,
        inherent_data_providers,
        other: (),
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

    let (network, network_status_sinks, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            on_demand: None,
            block_announce_validator_builder: None,
            finality_proof_request_builder: None,
            finality_proof_provider: None,
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            backend.clone(),
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let role = config.role.clone();
    let prometheus_registry = config.prometheus_registry().cloned();
    let telemetry_connection_sinks = sc_service::TelemetryConnectionSinks::default();

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = jupiter_rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
                grandpa: None,
            };

            jupiter_rpc::create_full(deps)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore.clone(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        telemetry_connection_sinks: telemetry_connection_sinks.clone(),
        rpc_extensions_builder: rpc_extensions_builder,
        on_demand: None,
        remote_blockchain: None,
        backend,
        network_status_sinks,
        system_rpc_tx,
        config,
    })?;

    if role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
        );

        let authorship_future = sc_consensus_manual_seal::run_instant_seal(
            Box::new(client.clone()),
            proposer,
            client,
            transaction_pool.pool().clone(),
            select_chain,
            inherent_data_providers,
        );

        task_manager
            .spawn_essential_handle()
            .spawn_blocking("instant-seal", authorship_future);
    };

    network_starter.start_network();
    Ok(task_manager)
}

/// Builds a new service for a light client.
pub fn new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
    let (client, backend, keystore, mut task_manager, on_demand) =
        sc_service::new_light_parts::<Block, RuntimeApi, Executor>(&config)?;

    let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
        config.transaction_pool.clone(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
        on_demand.clone(),
    ));

    let import_queue = sc_consensus_manual_seal::import_queue(
        Box::new(client.clone()),
        &task_manager.spawn_handle(),
        config.prometheus_registry(),
    );

    let fprb = Box::new(DummyFinalityProofRequestBuilder::default()) as Box<_>;

    let finality_proof_provider =
        GrandpaFinalityProofProvider::new_for_service(backend.clone(), client.clone());

    let (network, network_status_sinks, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            on_demand: Some(on_demand.clone()),
            block_announce_validator_builder: None,
            finality_proof_request_builder: Some(fprb),
            finality_proof_provider: Some(finality_proof_provider),
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            backend.clone(),
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        remote_blockchain: Some(backend.remote_blockchain()),
        transaction_pool,
        task_manager: &mut task_manager,
        on_demand: Some(on_demand),
        rpc_extensions_builder: Box::new(|_, _| ()),
        telemetry_connection_sinks: sc_service::TelemetryConnectionSinks::default(),
        config,
        client,
        keystore,
        backend,
        network,
        network_status_sinks,
        system_rpc_tx,
    })?;

    network_starter.start_network();

    Ok(task_manager)
}
