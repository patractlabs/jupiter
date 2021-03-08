#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::collections::btree_map::BTreeMap;

pub use pallet_poa::AuthorityState;

sp_api::decl_runtime_apis! {
    pub trait PoaApi<AccountId: Codec + Ord>{
        fn authorities() -> BTreeMap<AccountId, AuthorityState>;
    }
}
