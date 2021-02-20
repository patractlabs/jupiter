//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
use std::sync::Arc;
use cumulus_client_consensus_relay_chain::{
    build_relay_chain_consensus, BuildRelayChainConsensusParams,
};
use cumulus_client_network::build_block_announce_validator;
use cumulus_primitives_core::ParaId;
use cumulus_client_service::{
    prepare_node_config, start_collator, start_full_node, StartCollatorParams, StartFullNodeParams,
};
use sc_telemetry::TelemetrySpan;
use polkadot_primitives::v0::CollatorPair;
use sp_core::Pair;

use sc_executor::native_executor_instance;
pub use sc_executor::NativeExecutor;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, Role, PartialComponents};

use jupiter_para_runtime::{self, RuntimeApi, OCW_DB_RANDOM, RpcPort};
use patract_primitives::Block;

use sc_client_api::Backend;
use sp_core::offchain::OffchainStorage;
use codec::Encode;

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
native_executor_instance!(
    pub Executor,
    jupiter_para_runtime::api::dispatch,
    jupiter_para_runtime::native_version,
    (frame_benchmarking::benchmarking::HostFunctions, patract_io::pairing::HostFunctions),
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
        (),
    >,
    ServiceError,
> {
    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
    let client = Arc::new(client);

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
    );

    let inherent_data_providers = sp_inherents::InherentDataProviders::new();

    let registry = config.prometheus_registry();

    let import_queue = cumulus_client_consensus_relay_chain::import_queue(
        client.clone(),
        client.clone(),
        inherent_data_providers.clone(),
        &task_manager.spawn_handle(),
        registry.clone(),
    )?;

    let params = PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        inherent_data_providers,
        select_chain: (),
        other: (),
    };

    Ok(params)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RB>(
    parachain_config: Configuration,
    collator_key: CollatorPair,
    polkadot_config: Configuration,
    id: ParaId,
    validator: bool,
    rpc_ext_builder: RB,
) -> sc_service::error::Result<(TaskManager, Arc<FullClient>)>
    where
        RB: Fn(
            Arc<FullClient>,
        ) -> patract_rpc::IoHandler
        + Send
        + 'static,
{
    if matches!(parachain_config.role, Role::Light) {
        return Err("Light client not supported!".into());
    }

    let parachain_config = prepare_node_config(parachain_config);

    let rpc_port = polkadot_config.rpc_http.clone();

    let polkadot_full_node =
        cumulus_client_service::build_polkadot_full_node(polkadot_config, collator_key.public())
            .map_err(|e| match e {
                polkadot_service::Error::Sub(x) => x,
                s => format!("{}", s).into(),
            })?;

    let params = new_partial(&parachain_config)?;
    params
        .inherent_data_providers
        .register_provider(sp_timestamp::InherentDataProvider)
        .unwrap();

    let client = params.client.clone();
    let backend = params.backend.clone();
    let block_announce_validator = build_block_announce_validator(
        polkadot_full_node.client.clone(),
        id,
        Box::new(polkadot_full_node.network.clone()),
        polkadot_full_node.backend.clone(),
    );

    let prometheus_registry = parachain_config.prometheus_registry().cloned();
    let transaction_pool = params.transaction_pool.clone();
    let mut task_manager = params.task_manager;
    let import_queue = params.import_queue;
    let (network, network_status_sinks, system_rpc_tx, start_network) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            on_demand: None,
            block_announce_validator_builder: Some(Box::new(|_| block_announce_validator)),
        })?;

    if parachain_config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &parachain_config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
        );
    }

    let rpc_client = client.clone();
    let rpc_extensions_builder = Box::new(move |_, _| rpc_ext_builder(rpc_client.clone()));

    let telemetry_span = TelemetrySpan::new();
    let _telemetry_span_entered = telemetry_span.enter();

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
        network_status_sinks,
        system_rpc_tx,
        telemetry_span: Some(telemetry_span.clone()),
    })?;

    let announce_block = {
        let network = network.clone();
        Arc::new(move |hash, data| network.announce_block(hash, Some(data)))
    };

    if validator {
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool,
            prometheus_registry.as_ref(),
        );
        let spawner = task_manager.spawn_handle();

        if let Some(rpc_port_addr) = rpc_port {
            let rpc_port = RpcPort{ 0: rpc_port_addr.to_string().into() };
            let offchain_storage = backend.offchain_storage();
            if let Some(mut offchain_storage) = offchain_storage {
                offchain_storage.set(sp_offchain::STORAGE_PREFIX, OCW_DB_RANDOM, &rpc_port.encode());
            }
        }

        let parachain_consensus = build_relay_chain_consensus(BuildRelayChainConsensusParams {
            para_id: id,
            proposer_factory,
            inherent_data_providers: params.inherent_data_providers,
            block_import: client.clone(),
            relay_chain_client: polkadot_full_node.client.clone(),
            relay_chain_backend: polkadot_full_node.backend.clone(),
        });

        let params = StartCollatorParams {
            para_id: id,
            block_status: client.clone(),
            announce_block,
            client: client.clone(),
            task_manager: &mut task_manager,
            collator_key,
            relay_chain_full_node: polkadot_full_node,
            spawner,
            backend,
            parachain_consensus,
        };

        start_collator(params).await?;
    } else {
        let params = StartFullNodeParams {
            client: client.clone(),
            announce_block,
            task_manager: &mut task_manager,
            para_id: id,
            polkadot_full_node,
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
