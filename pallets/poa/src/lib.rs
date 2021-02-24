#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::pallet_prelude::*;
use sp_runtime::{traits::StaticLookup, RuntimeDebug};
use sp_std::{collections::btree_map::BTreeMap, prelude::*};

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The minimum amount required to keep an account open.
        #[pallet::constant]
        type MinimumAuthorities: Get<u32>;
        ///
        type CouncilOrigin: EnsureOrigin<Self::Origin>;
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The authority account id already exists in authority list.
        Existed,
        /// The authority account id is not existed in authority list.
        NotExisted,
        /// Reach minimal authorities count.
        TooLowAuthorities,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
    pub enum Event<T: Config> {
        /// Add a new authority.
        AddAuthority(T::AccountId),
        /// Remove a new authority.
        RemoveAuthority(T::AccountId),
    }

    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub type Authorities<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AuthorityState, OptionQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub init_authorities: Vec<T::AccountId>,
    }
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                init_authorities: Default::default(),
            }
        }
    }
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            if (self.init_authorities.len() as u32) < T::MinimumAuthorities::get() {
                panic!("poa init authorities must larger than `MinimumAuthorities` requirement");
            }
            for a in self.init_authorities.iter() {
                Authorities::<T>::insert(a.clone(), AuthorityState::Working);
            }
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::add_authority())]
        pub fn add_authority(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            T::CouncilOrigin::ensure_origin(origin)?;
            let new_authority = T::Lookup::lookup(dest)?;
            ensure!(
                Self::authorities(&new_authority).is_none(),
                Error::<T>::Existed
            );

            Authorities::<T>::insert(&new_authority, AuthorityState::Working);

            Self::deposit_event(Event::AddAuthority(new_authority));
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::remove_authority())]
        pub fn remove_authority(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            T::CouncilOrigin::ensure_origin(origin)?;
            let old_authority = T::Lookup::lookup(dest)?;
            // check minimal
            ensure!(
                Self::working_authorities().len() as u32 > T::MinimumAuthorities::get(),
                Error::<T>::TooLowAuthorities
            );

            match Authorities::<T>::take(&old_authority) {
                Some(_) => {
                    Self::deposit_event(Event::RemoveAuthority(old_authority));
                    Ok(().into())
                }
                None => Err(Error::<T>::NotExisted.into()),
            }
        }
    }
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AuthorityState {
    Working,
    Waiting,
}

impl<T: Config> Pallet<T> {
    pub fn working_authorities() -> Vec<T::AccountId> {
        Authorities::<T>::iter()
            .filter_map(|(a, s)| {
                if let AuthorityState::Working = s {
                    Some(a)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn authorities_map() -> BTreeMap<T::AccountId, AuthorityState> {
        Authorities::<T>::iter().collect()
    }
}
