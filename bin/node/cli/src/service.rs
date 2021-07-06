//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
use cumulus_client_consensus_relay_chain::{
    build_relay_chain_consensus, BuildRelayChainConsensusParams,
};
use cumulus_client_network::build_block_announce_validator;
use cumulus_client_service::{
    prepare_node_config, start_collator, start_full_node, StartCollatorParams, StartFullNodeParams,
};
use cumulus_primitives_core::ParaId;
use polkadot_primitives::v0::CollatorPair;
use sc_telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle};
use std::sync::Arc;

use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_service::{
    error::Error as ServiceError, Configuration, PartialComponents, Role, TaskManager,
};

use jupiter_primitives::Block;
use jupiter_runtime::{self, RpcPort, RuntimeApi, OCW_DB_RANDOM};

use codec::Encode;
use sc_client_api::Backend;
use sp_core::offchain::OffchainStorage;

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
native_executor_instance!(
    pub Executor,
    jupiter_runtime::api::dispatch,
    jupiter_runtime::native_version,
    (frame_benchmarking::benchmarking::HostFunctions, jupiter_io::pairing::HostFunctions),
);

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;

pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        (),
        sp_consensus::DefaultImportQueue<Block, FullClient>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (Option<Telemetry>, Option<TelemetryWorkerHandle>),
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

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, Executor>(
            &config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
        )?;
    let client = Arc::new(client);

    let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager.spawn_handle().spawn("telemetry", worker.run());
        telemetry
    });

    let registry = config.prometheus_registry();

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    // use the cumulus consensus with relay chain, this is for rococo-v1
    // todo: use aura related import_queue implementation for 0.9.3+, plz ref to cumulus
    let import_queue = cumulus_client_consensus_relay_chain::import_queue(
        client.clone(),
        client.clone(),
        |_, _| async { Ok(sp_timestamp::InherentDataProvider::from_system_time()) },
        &task_manager.spawn_essential_handle(),
        registry.clone(),
    )?;

    let params = PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: (),
        other: (telemetry, telemetry_worker_handle),
    };

    Ok(params)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RB>(
    parachain_config: Configuration,
    _collator_key: CollatorPair,
    polkadot_config: Configuration,
    id: ParaId,
    validator: bool,
    rpc_ext_builder: RB,
) -> sc_service::error::Result<(TaskManager, Arc<FullClient>)>
where
    RB: Fn(Arc<FullClient>) -> jupiter_rpc::IoHandler + Send + 'static,
{
    if matches!(parachain_config.role, Role::Light) {
        return Err("Light client not supported!".into());
    }

    let parachain_config = prepare_node_config(parachain_config);

    let rpc_port = polkadot_config.rpc_http.clone();

    let params = new_partial(&parachain_config)?;
    let (mut telemetry, telemetry_worker_handle) = params.other;

    let relay_chain_full_node = cumulus_client_service::build_polkadot_full_node(
        polkadot_config,
        telemetry_worker_handle,
    )
    .map_err(|e| match e {
        polkadot_service::Error::Sub(x) => x,
        s => format!("{}", s).into(),
    })?;

    let client = params.client.clone();
    let backend = params.backend.clone();
    let block_announce_validator = build_block_announce_validator(
        relay_chain_full_node.client.clone(),
        id,
        Box::new(relay_chain_full_node.network.clone()),
        relay_chain_full_node.backend.clone(),
    );

    let prometheus_registry = parachain_config.prometheus_registry().cloned();
    let transaction_pool = params.transaction_pool.clone();
    let mut task_manager = params.task_manager;
    // wrapper import queue from new_partial function with SharedImportQueue
    let import_queue = cumulus_client_service::SharedImportQueue::new(params.import_queue);
    let (network, system_rpc_tx, start_network) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue: import_queue.clone(),
            on_demand: None,
            block_announce_validator_builder: Some(Box::new(|_| block_announce_validator)),
        })?;

    if parachain_config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &parachain_config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    // the passing rpc_ext_builder current is default, should add contract and related rpc
    // NOTICE: the node-pre module already use rpc, but here we may need some different rpc module
    let rpc_client = client.clone();
    let rpc_transaction_pool = transaction_pool.clone();
    // let rpc_extensions_builder = Box::new(move |_, _| rpc_ext_builder(rpc_client.clone()));
    let rpc_extensions_builder = {
        let client = rpc_client.clone();
        let pool = rpc_transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = jupiter_rpc::BasicDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
            };
            jupiter_rpc::create_basic(deps)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        on_demand: None,
        remote_blockchain: None,
        rpc_extensions_builder,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        config: parachain_config,
        keystore: params.keystore_container.sync_keystore(),
        backend: backend.clone(),
        network: network.clone(),
        system_rpc_tx,
        telemetry: telemetry.as_mut(),
    })?;

    let announce_block = {
        let network = network.clone();
        Arc::new(move |hash, data| network.announce_block(hash, data))
    };

    if validator {
        let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool,
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );
        let spawner = task_manager.spawn_handle();

        if let Some(rpc_port_addr) = rpc_port {
            let rpc_port = RpcPort {
                0: rpc_port_addr.to_string().into(),
            };
            let offchain_storage = backend.offchain_storage();
            if let Some(mut offchain_storage) = offchain_storage {
                offchain_storage.set(
                    sp_offchain::STORAGE_PREFIX,
                    OCW_DB_RANDOM,
                    &rpc_port.encode(),
                );
            }
        }

        // this default inherent data provider should not used here.
        // let create_inherent_data_providers = move |_, _| async {
        //     Ok(sp_timestamp::InherentDataProvider::from_system_time())
        // };

        // the inherent data provider contains ParachainInherentData and timestamp
        let relay_chain_backend = relay_chain_full_node.backend.clone();
        let relay_chain_client = relay_chain_full_node.client.clone();

        let create_inherent_data_providers = move |_, (relay_parent, validation_data)| {
            let parachain_inherent =
                cumulus_primitives_parachain_inherent::ParachainInherentData::create_at_with_client(
                    relay_parent,
                    &relay_chain_client,
                    &*relay_chain_backend,
                    &validation_data,
                    id,
                );
            async move {
                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
                let parachain_inherent =
                    parachain_inherent.ok_or_else(|| {
                        Box::<dyn std::error::Error + Send + Sync>::from(
                            "Failed to create parachain inherent",
                        )
                    })?;
                Ok((timestamp, parachain_inherent))
            }
        };

        let parachain_consensus = build_relay_chain_consensus(BuildRelayChainConsensusParams {
            para_id: id,
            proposer_factory,
            create_inherent_data_providers,
            block_import: client.clone(),
            relay_chain_client: relay_chain_full_node.client.clone(),
            relay_chain_backend: relay_chain_full_node.backend.clone(),
        });

        let params = StartCollatorParams {
            para_id: id,
            block_status: client.clone(),
            announce_block,
            client: client.clone(),
            task_manager: &mut task_manager,
            relay_chain_full_node,
            spawner,
            parachain_consensus,
            import_queue,
        };

        start_collator(params).await?;
    } else {
        let params = StartFullNodeParams {
            client: client.clone(),
            announce_block,
            task_manager: &mut task_manager,
            para_id: id,
            relay_chain_full_node,
        };

        start_full_node(params)?;
    }

    start_network.start_network();

    Ok((task_manager, client))
}

/// Start a normal parachain node.
pub async fn start_node(
    parachain_config: Configuration,
    collator_key: CollatorPair,
    polkadot_config: Configuration,
    id: ParaId,
    validator: bool,
) -> sc_service::error::Result<(TaskManager, Arc<FullClient>)> {
    start_node_impl(
        parachain_config,
        collator_key,
        polkadot_config,
        id,
        validator,
        |_| Default::default(),
    )
    .await
}
