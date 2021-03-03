#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod offence;
mod session;
mod slashing;
pub mod weights;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::pallet_prelude::*;
use sp_runtime::{traits::StaticLookup, RuntimeDebug, SaturatedConversion};
use sp_std::{collections::btree_map::BTreeMap, prelude::*};

pub use pallet::*;
pub use session::SimpleValidatorIdConverter;
pub use weights::WeightInfo;

pub type EraIndex = u32;
pub(crate) const LOG_TARGET: &'static str = "poa";

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_staking::SessionIndex;

    use super::*;
    use frame_support::traits::UnixTime;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The minimum amount required to keep an account open.
        #[pallet::constant]
        type MinimumAuthorities: Get<u32>;
        #[pallet::constant]
        /// Number of sessions per era.
        type SessionsPerEra: Get<SessionIndex>;
        /// Number of eras that slashes are deferred by, after computation.
        ///
        /// Set to 0 if slashes should be applied immediately, without opportunity for intervention.
        #[pallet::constant]
        type SlashDeferDuration: Get<EraIndex>;
        /// Time used for computing era duration.
        ///
        /// It is guaranteed to start being called from the first `on_finalize`. Thus value at genesis
        /// is not used.
        type UnixTime: UnixTime;
        ///
        type CouncilOrigin: EnsureOrigin<Self::Origin>;
        /// The origin which can cancel a deferred slash. Root can always do this.
        type SlashCancelOrigin: EnsureOrigin<Self::Origin>;
        /// Interface for interacting with a session module.
        type SessionInterface: crate::session::SessionInterface<Self::AccountId>;
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
        /// Targets cannot be empty.
        EmptyTargets,
        /// Slash record index out of bounds.
        InvalidSlashIndex,
        /// Items are not sorted and unique.
        NotSortedAndUnique,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
    pub enum Event<T: Config> {
        /// Add a new authority.
        AddAuthority(T::AccountId),
        /// Remove a new authority.
        RemoveAuthority(T::AccountId),

        NewAuthorities,

        TooLowAuthorities,

        Offence(T::AccountId),
        Slash(T::AccountId),
    }

    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub type Authorities<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AuthorityState, OptionQuery>;
    #[pallet::storage]
    #[pallet::getter(fn minimum_authority_count)]
    pub type MinimumAuthorityCount<T: Config> =
        StorageValue<_, u32, ValueQuery, MinimumAuthorityCountDefault>;
    #[pallet::type_value]
    pub fn MinimumAuthorityCountDefault() -> u32 {
        1
    }

    /// Number of eras to keep in history.
    ///
    /// Information is kept for eras in `[current_era - history_depth; current_era]`.
    ///
    /// Must be more than the number of eras delayed by session otherwise. I.e. active era must
    /// always be in history. I.e. `active_era > current_era - history_depth` must be
    /// guaranteed.
    #[pallet::storage]
    #[pallet::getter(fn history_depth)]
    pub(crate) type HistoryDepth<T: Config> = StorageValue<_, u32, ValueQuery, HistoryDepthDefault>;
    #[pallet::type_value]
    pub(crate) fn HistoryDepthDefault() -> u32 {
        24
    }

    /// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
    /// easy to initialize and the performance hit is minimal (we expect no more than four
    /// invulnerables) and restricted to testnets.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    pub type Invulnerables<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// The current era index.
    ///
    /// This is the latest planned era, depending on how the Session pallet queues the validator
    /// set, it might be active or not.
    #[pallet::storage]
    #[pallet::getter(fn current_era)]
    pub type CurrentEra<T: Config> = StorageValue<_, EraIndex, OptionQuery>;
    /// The active era information, it holds index and start.
    ///
    /// The active era is the era being currently rewarded. Validator set of this era must be
    /// equal to [`SessionInterface::validators`].
    #[pallet::storage]
    #[pallet::getter(fn active_era)]
    pub type ActiveEra<T: Config> = StorageValue<_, ActiveEraInfo, OptionQuery>;
    /// The session index at which the era start for the last `HISTORY_DEPTH` eras.
    ///
    /// Note: This tracks the starting session (i.e. session index when era start being active)
    /// for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`.
    #[pallet::storage]
    #[pallet::getter(fn eras_start_session_index)]
    pub type ErasStartSessionIndex<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, SessionIndex, OptionQuery>;
    /// Mode of era forcing.
    #[pallet::storage]
    #[pallet::getter(fn force_era)]
    pub type ForceEra<T: Config> = StorageValue<_, Forcing, ValueQuery>;
    /// All unapplied slashes that are queued for later.
    #[pallet::storage]
    #[pallet::getter(fn unapplied_slashes)]
    pub type UnappliedSlashes<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, Vec<UnappliedSlash<T::AccountId>>, ValueQuery>;
    /// helper for record waiting slashes
    #[pallet::storage]
    #[pallet::getter(fn waiting_slashes)]
    pub type WaitingSlashes<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u8, ValueQuery>;

    /// The earliest era for which we have a pending, unapplied slash.
    #[pallet::storage]
    pub type EarliestUnappliedSlash<T: Config> = StorageValue<_, EraIndex, OptionQuery>;
    // ------------ Pallet -----------
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub minimum_authority_count: u32,
        pub init_authorities: Vec<T::AccountId>,
        pub init_invulnerables: Vec<T::AccountId>,
        pub force_era: Forcing,
    }
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                minimum_authority_count: 1,
                init_authorities: Default::default(),
                init_invulnerables: Default::default(),
                force_era: Default::default(),
            }
        }
    }
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            MinimumAuthorityCount::<T>::put(self.minimum_authority_count);
            if (self.init_authorities.len() as u32) < T::MinimumAuthorities::get() {
                panic!("poa init authorities must larger than `MinimumAuthorities` requirement");
            }
            for a in self.init_authorities.iter() {
                Authorities::<T>::insert(a.clone(), AuthorityState::Working);
            }

            let mut invulnerables_list = Vec::new();
            for a in self.init_invulnerables.iter() {
                invulnerables_list.push(a.clone());
            }
            Invulnerables::<T>::put(invulnerables_list);

            ForceEra::<T>::put(self.force_era)
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: T::BlockNumber) {
            // Set the start of the first era.
            if let Some(mut active_era) = Self::active_era() {
                if active_era.start.is_none() {
                    let now_as_millis_u64 = T::UnixTime::now().as_millis().saturated_into::<u64>();
                    active_era.start = Some(now_as_millis_u64);
                    // This write only ever happens once, we don't include it in the weight in general
                    ActiveEra::<T>::put(active_era);
                }
            }
        }
    }

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

        #[pallet::weight(T::WeightInfo::set_invulnerables(invulnerables.len() as u32))]
        pub fn set_invulnerables(
            origin: OriginFor<T>,
            invulnerables: Vec<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <Invulnerables<T>>::put(invulnerables);
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::cancel_deferred_slash(slash_indices.len() as u32))]
        pub fn cancel_deferred_slash(
            origin: OriginFor<T>,
            era: EraIndex,
            slash_indices: Vec<u32>,
        ) -> DispatchResultWithPostInfo {
            T::SlashCancelOrigin::ensure_origin(origin)?;

            ensure!(!slash_indices.is_empty(), Error::<T>::EmptyTargets);
            ensure!(
                is_sorted_and_unique(&slash_indices),
                Error::<T>::NotSortedAndUnique
            );

            let mut unapplied = UnappliedSlashes::<T>::get(&era);
            let last_item = slash_indices[slash_indices.len() - 1];
            ensure!(
                (last_item as usize) < unapplied.len(),
                Error::<T>::InvalidSlashIndex
            );

            for (removed, index) in slash_indices.into_iter().enumerate() {
                let index = (index as usize) - removed;
                let validator = unapplied[index].validator.clone();
                WaitingSlashes::<T>::mutate(validator.clone(), |waiting_slash| {
                    *waiting_slash = *waiting_slash - 1;
                    if *waiting_slash == 0 {
                        Authorities::<T>::mutate(validator, |state| {
                            if let Some(ref mut state) = state {
                                *state = AuthorityState::Working;
                            }
                        })
                    }
                });

                unapplied.remove(index);
            }

            UnappliedSlashes::<T>::insert(&era, &unapplied);
            Ok(().into())
        }

        /// Force there to be no new eras indefinitely.
		///
		/// The dispatch origin must be Root.
		///
		/// # <weight>
		/// - No arguments.
		/// - Weight: O(1)
		/// - Write: ForceEra
		/// # </weight>
		#[pallet::weight(T::WeightInfo::force_no_eras())]
        pub fn force_no_eras(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ForceEra::<T>::put(Forcing::ForceNone);
            Ok(().into())
        }

        /// Force there to be a new era at the end of the next session. After this, it will be
		/// reset to normal (non-forced) behaviour.
		///
		/// The dispatch origin must be Root.
		///
		/// # <weight>
		/// - No arguments.
		/// - Weight: O(1)
		/// - Write ForceEra
		/// # </weight>
        #[pallet::weight(T::WeightInfo::force_new_era())]
        pub fn force_new_era(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ForceEra::<T>::put(Forcing::ForceNew);
            Ok(().into())
        }

        /// Force there to be a new era at the end of sessions indefinitely.
		///
		/// The dispatch origin must be Root.
		///
		/// # <weight>
		/// - Weight: O(1)
		/// - Write: ForceEra
		/// # </weight>
        #[pallet::weight(T::WeightInfo::force_new_era_always())]
        pub fn force_new_era_always(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ForceEra::<T>::put(Forcing::ForceAlways);
            Ok(().into())
        }

        /// Set `HistoryDepth` value. This function will delete any history information
		/// when `HistoryDepth` is reduced.
		///
		/// Parameters:
		/// - `new_history_depth`: The new history depth you would like to set.
		/// - `era_items_deleted`: The number of items that will be deleted by this dispatch.
		///    This should report all the storage items that will be deleted by clearing old
		///    era history. Needed to report an accurate weight for the dispatch. Trusted by
		///    `Root` to report an accurate number.
		///
		/// Origin must be root.
		///
		/// # <weight>
		/// - E: Number of history depths removed, i.e. 10 -> 7 = 3
		/// - Weight: O(E)
		/// - DB Weight:
		///     - Reads: Current Era, History Depth
		///     - Writes: History Depth
		///     - Clear Prefix Each: Era Stakers, EraStakersClipped, ErasValidatorPrefs
		///     - Writes Each: ErasValidatorReward, ErasRewardPoints, ErasTotalStake, ErasStartSessionIndex
		/// # </weight>
        #[pallet::weight(T::WeightInfo::set_history_depth(*_era_items_deleted))]
        pub fn set_history_depth(origin: OriginFor<T>,
                             new_history_depth: EraIndex,
                             _era_items_deleted: u32,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            if let Some(current_era) = Self::current_era() {
                HistoryDepth::<T>::mutate(|history_depth| {
                    let last_kept = current_era.checked_sub(*history_depth).unwrap_or(0);
                    let new_last_kept = current_era.checked_sub(new_history_depth).unwrap_or(0);
                    for era_index in last_kept..new_last_kept {
                        ErasStartSessionIndex::<T>::remove(era_index);
                    }
                    *history_depth = new_history_depth
                })
            }
            Ok(().into())
        }
    }

    /// Check that list is sorted and has no duplicates.
    fn is_sorted_and_unique(list: &[u32]) -> bool {
        list.windows(2).all(|w| w[0] < w[1])
    }
}

/// Mode of era-forcing.
#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Forcing {
    /// Not forcing anything - just let whatever happen.
    NotForcing,
    /// Force a new era, then reset to `NotForcing` as soon as it is done.
    ForceNew,
    /// Avoid a new era indefinitely.
    ForceNone,
    /// Force a new era at the end of all sessions indefinitely.
    ForceAlways,
}

impl Default for Forcing {
    fn default() -> Self {
        Forcing::NotForcing
    }
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AuthorityState {
    Working,
    Waiting,
}

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, RuntimeDebug, PartialEq)]
pub struct ActiveEraInfo {
    /// Index of era.
    pub index: EraIndex,
    /// Moment of start expressed as millisecond from `$UNIX_EPOCH`.
    ///
    /// Start can be none if start hasn't been set for the era yet,
    /// Start is set on the first on_finalize of the era to guarantee usage of `Time`.
    start: Option<u64>,
}

/// A pending slash record. The value of the slash has been computed but not applied yet,
/// rather deferred for several eras.
#[derive(Encode, Decode, Default, RuntimeDebug, PartialEq)]
pub struct UnappliedSlash<AccountId> {
    /// The stash ID of the offending validator.
    validator: AccountId,
    /// Reporters of the offence; bounty payout recipients.
    reporters: Vec<AccountId>,
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

    fn set_waiting(who: &T::AccountId) -> Option<()> {
        Authorities::<T>::mutate(who, |state| {
            if let Some(ref mut state) = state {
                *state = AuthorityState::Waiting;
                Some(())
            } else {
                None
            }
        })
    }

    /// limit minimal validators
    fn remove_from_authorities(who: &T::AccountId) {
        if Self::authorities_map().len() as u32 == MinimumAuthorityCount::<T>::get() {
            debug::warn!(
                target: LOG_TARGET,
                "Can not remove {} due to minimal authority count.", who
            );
            Self::deposit_event(Event::TooLowAuthorities);
        } else if Authorities::<T>::contains_key(who) {
            Authorities::<T>::remove(who);
            WaitingSlashes::<T>::remove(who);
            Self::deposit_event(Event::<T>::Slash(who.clone()));
        }
    }

    /// Ensures that at the end of the current session there will be a new era.
    fn ensure_new_era() {
        match ForceEra::<T>::get() {
            Forcing::ForceAlways | Forcing::ForceNew => (),
            _ => ForceEra::<T>::put(Forcing::ForceNew),
        }
    }
}
