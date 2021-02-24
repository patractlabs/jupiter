//! Node-specific RPC methods for interaction with contracts.

use std::sync::Arc;

use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

use std::collections::BTreeMap;

pub use pallet_poa_rpc_runtime_api::{AuthorityState, PoaApi as PoaRuntimeApi};

const RUNTIME_ERROR: i64 = 1;

/// PoA RPC methods.
#[rpc]
pub trait PoaApi<BlockHash, AccountId>
where
    AccountId: Ord,
{
    #[rpc(name = "poa_authorities")]
    fn authorities(&self, at: Option<BlockHash>) -> Result<BTreeMap<AccountId, AuthorityState>>;
}

/// An implementation of contract specific RPC methods.
pub struct PoA<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> PoA<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        PoA {
            client,
            _marker: Default::default(),
        }
    }
}
impl<C, Block, AccountId> PoaApi<<Block as BlockT>::Hash, AccountId> for PoA<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PoaRuntimeApi<Block, AccountId>,
    AccountId: Codec + Ord,
{
    fn authorities(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<BTreeMap<AccountId, AuthorityState>> {
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        let api = self.client.runtime_api();
        let authorities = api.authorities(&at).map_err(runtime_error_into_rpc_err)?;

        Ok(authorities)
    }
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime trapped".into(),
        data: Some(format!("{:?}", err).into()),
    }
}
