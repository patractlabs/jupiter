#![cfg_attr(not(feature = "std"), no_std)]

mod default_weights;

use codec::{Decode, Encode};

use sp_core::crypto::UncheckedFrom;
use sp_runtime::{
    traits::{Hash, SaturatedConversion},
    RuntimeDebug,
};
use sp_std::prelude::*;

use frame_support::{decl_error, decl_event, decl_module, decl_storage, weights::Weight};
use frame_system::ensure_signed;

use pallet_contracts::{CodeHash, ContractAddressFor};
#[cfg(feature = "megaclite")]
use pallet_contracts_megaclite::{
    CodeHash as MegacliteCodeHash, ContractAddressFor as MegacliteContractAddressFor,
};

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type WeightInfo: WeightInfo;
}

decl_error! {
    pub enum Error for Module<T: Trait> {

    }
}

decl_event! {
    pub enum Event<T> where
        <T as frame_system::Trait>::AccountId,
    {
        SetAccountGeneratedAddressType(AccountId, GeneratedContractAddressType),
    }
}

pub trait WeightInfo {
    fn set_generated_address_type() -> Weight;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = T::WeightInfo::set_generated_address_type()]
        pub fn set_generated_address_type(origin, address_type: GeneratedContractAddressType) {
            let who = ensure_signed(origin)?;
            AccountGeneratedAddressType::<T>::insert(&who, address_type);
            Self::deposit_event(RawEvent::SetAccountGeneratedAddressType(who, address_type));
        }
    }
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum GeneratedContractAddressType {
    Original,
    Repeatable,
}

impl Default for GeneratedContractAddressType {
    fn default() -> Self {
        GeneratedContractAddressType::Original
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as ContractsExt {
        pub AccountGeneratedAddressType get(fn account_generated_address_type):
            map hasher(blake2_128_concat) T::AccountId => GeneratedContractAddressType;
    }
}

impl<T: Trait> ContractAddressFor<CodeHash<T>, T::AccountId> for Module<T>
where
    T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
{
    fn contract_address_for(
        code_hash: &CodeHash<T>,
        data: &[u8],
        origin: &T::AccountId,
    ) -> T::AccountId {
        let data_hash = T::Hashing::hash(data);

        let mut buf = Vec::new();
        buf.extend_from_slice(code_hash.as_ref());
        buf.extend_from_slice(data_hash.as_ref());
        buf.extend_from_slice(origin.as_ref());

        match Self::account_generated_address_type(origin) {
            GeneratedContractAddressType::Original => { /*do nothing*/ }
            GeneratedContractAddressType::Repeatable => {
                let nonce = frame_system::Module::<T>::account_nonce(origin);
                let nonce: u32 = nonce.saturated_into::<u32>();
                buf.extend_from_slice(&nonce.to_le_bytes()[..]);
            }
        }

        UncheckedFrom::unchecked_from(T::Hashing::hash(&buf[..]))
    }
}

#[cfg(feature = "megaclite")]
impl<T: Trait> MegacliteContractAddressFor<MegacliteCodeHash<T>, T::AccountId> for Module<T>
where
    T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
{
    fn contract_address_for(
        code_hash: &MegacliteCodeHash<T>,
        data: &[u8],
        origin: &T::AccountId,
    ) -> T::AccountId {
        let data_hash = T::Hashing::hash(data);

        let mut buf = Vec::new();
        buf.extend_from_slice(code_hash.as_ref());
        buf.extend_from_slice(data_hash.as_ref());
        buf.extend_from_slice(origin.as_ref());

        match Self::account_generated_address_type(origin) {
            GeneratedContractAddressType::Original => { /*do nothing*/ }
            GeneratedContractAddressType::Repeatable => {
                let nonce = frame_system::Module::<T>::account_nonce(origin);
                let nonce: u32 = nonce.saturated_into::<u32>();
                buf.extend_from_slice(&nonce.to_le_bytes()[..]);
            }
        }

        UncheckedFrom::unchecked_from(T::Hashing::hash(&buf[..]))
    }
}
