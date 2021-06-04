// This file is part of Substrate. In frame/staking/src/lib.rs

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// Copyright (C) 2021-2021 Patract Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use frame_support::traits::Get;
use pallet_session::historical;
use sp_runtime::traits::Convert;
use sp_staking::SessionIndex;
use sp_std::prelude::*;

use crate::slashing;
use crate::{
    ActiveEra, ActiveEraInfo, Config, CurrentEra, EarliestUnappliedSlash, EraIndex,
    ErasStartSessionIndex, Event, ForceEra, Forcing, Pallet, UnappliedSlashes, LOG_TARGET,
};

impl<T: Config> Pallet<T> {
    fn new_session(session_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        if let Some(current_era) = Self::current_era() {
            // Initial era has been set.

            let current_era_start_session_index = Self::eras_start_session_index(current_era)
                .unwrap_or_else(|| {
                    frame_support::print("Error: start_session_index must be set for current_era");
                    0
                });

            let era_length = session_index
                .checked_sub(current_era_start_session_index)
                .unwrap_or(0); // Must never happen.

            match Self::force_era() {
                Forcing::NotForcing if era_length >= T::SessionsPerEra::get() => (),
                Forcing::ForceNew => ForceEra::<T>::kill(),
                Forcing::ForceAlways => (),
                _ => return None,
            }

            // new era.
            Self::new_era(session_index)
        } else {
            // Set initial era
            Self::new_era(session_index)
        }
    }
    /// Plan a new era. Return the potential new staking set.
    fn new_era(start_session_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        // Increment or set current era.
        let current_era = CurrentEra::<T>::mutate(|s| {
            *s = Some(s.map(|s| s + 1).unwrap_or(0));
            s.unwrap()
        });
        ErasStartSessionIndex::<T>::insert(&current_era, &start_session_index);

        // Clean old era information.
        if let Some(old_era) = current_era.checked_sub(Self::history_depth() + 1) {
            Self::clear_era_information(old_era);
        }

        // Set staking information for new era.
        let maybe_new_validators = Self::select_and_update_validators(current_era);

        maybe_new_validators
    }

    /// Start a session potentially starting an era.
    fn start_session(start_session: SessionIndex) {
        let next_active_era = Self::active_era().map(|e| e.index + 1).unwrap_or(0);
        // This is only `Some` when current era has already progressed to the next era, while the
        // active era is one behind (i.e. in the *last session of the active era*, or *first session
        // of the new current era*, depending on how you look at it).
        if let Some(next_active_era_start_session_index) =
            Self::eras_start_session_index(next_active_era)
        {
            if next_active_era_start_session_index == start_session {
                Self::start_era(start_session);
            } else if next_active_era_start_session_index < start_session {
                // This arm should never happen, but better handle it than to stall the staking
                // pallet.
                frame_support::print("Warning: A session appears to have been skipped.");
                Self::start_era(start_session);
            }
        }
    }

    /// End a session potentially ending an era.
    fn end_session(session_index: SessionIndex) {
        if let Some(active_era) = Self::active_era() {
            if let Some(next_active_era_start_session_index) =
                Self::eras_start_session_index(active_era.index + 1)
            {
                if next_active_era_start_session_index == session_index + 1 {
                    Self::end_era(active_era, session_index);
                }
            }
        }
    }

    /// * Increment `active_era.index`,
    /// * reset `active_era.start`,
    fn start_era(start_session: SessionIndex) {
        let active_era = ActiveEra::<T>::mutate(|active_era| {
            let new_index = active_era.as_ref().map(|info| info.index + 1).unwrap_or(0);
            *active_era = Some(ActiveEraInfo {
                index: new_index,
                // Set new active era start in next `on_finalize`. To guarantee usage of `Time`
                start: None,
            });
            new_index
        });

        // TODO set clean up scope in future
        if start_session > 24 * 7 {
            T::SessionInterface::prune_historical_up_to(start_session - 24 * 7);
        }

        Self::apply_unapplied_slashes(active_era);
    }

    fn end_era(_active_era: ActiveEraInfo, _session_index: SessionIndex) {
        // currently poa do not need payment, thus do nothing here
    }

    fn clear_era_information(_era_index: EraIndex) {
        // place holder
    }

    /// Apply previously-unapplied slashes on the beginning of a new era, after a delay.
    fn apply_unapplied_slashes(active_era: EraIndex) {
        let slash_defer_duration = T::SlashDeferDuration::get();
        EarliestUnappliedSlash::<T>::mutate(|earliest| {
            if let Some(ref mut earliest) = earliest {
                let keep_from = active_era.saturating_sub(slash_defer_duration);
                for era in (*earliest)..keep_from {
                    let era_slashes = UnappliedSlashes::<T>::take(&era);
                    for slash in era_slashes {
                        slashing::apply_slash::<T>(slash.validator);
                    }
                }

                *earliest = (*earliest).max(keep_from)
            }
        })
    }

    fn select_and_update_validators(current_era: EraIndex) -> Option<Vec<T::AccountId>> {
        let mut validators = T::SessionInterface::validators();
        // sort old list, in case other problem,
        validators.sort();
        let mut new_validators = Self::working_authorities();
        new_validators.sort();

        if validators == new_validators {
            log::info!(target: LOG_TARGET, "same validators, do nothing");
            None
        } else if (new_validators.len() as u32) < T::MinimumAuthorities::get() {
            log::warn!(
                target: LOG_TARGET,
                "new validator collection is less then required count|new_validator:{:?}|required:{:}",
                new_validators,
                T::MinimumAuthorities::get()
            );
            Self::deposit_event(Event::<T>::TooLowAuthorities);
            None
        } else {
            // emit event
            Self::deposit_event(Event::<T>::NewAuthorities);
            log::info!(
                target: LOG_TARGET,
                "💸 new validator set of size {:?} has been selected for era {:?}",
                new_validators.len(),
                current_era,
            );
            Some(new_validators)
        }
    }
}

impl<T: Config> pallet_session::SessionManager<T::AccountId> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        log::trace!(
            target: LOG_TARGET,
            "[{:?}] planning new_session({})",
            <frame_system::Pallet<T>>::block_number(),
            new_index
        );
        Self::new_session(new_index)
    }
    fn end_session(end_index: SessionIndex) {
        log::trace!(
            target: LOG_TARGET,
            "[{:?}] ending end_session({})",
            <frame_system::Pallet<T>>::block_number(),
            end_index
        );
        Self::end_session(end_index)
    }
    fn start_session(start_index: SessionIndex) {
        log::trace!(
            target: LOG_TARGET,
            "[{:?}] starting start_session({})",
            <frame_system::Pallet<T>>::block_number(),
            start_index
        );
        Self::start_session(start_index)
    }
}

impl<T: Config> historical::SessionManager<T::AccountId, T::AccountId> for Pallet<T> {
    fn new_session(new_index: SessionIndex) -> Option<Vec<(T::AccountId, T::AccountId)>> {
        <Self as pallet_session::SessionManager<_>>::new_session(new_index)
            .map(|validators| validators.into_iter().map(|v| (v.clone(), v)).collect())
    }
    fn start_session(start_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::start_session(start_index)
    }
    fn end_session(end_index: SessionIndex) {
        <Self as pallet_session::SessionManager<_>>::end_session(end_index)
    }
}

pub struct SimpleValidatorIdConverter<AccountId>(sp_std::marker::PhantomData<AccountId>);

impl<AccountId> Convert<AccountId, Option<AccountId>> for SimpleValidatorIdConverter<AccountId> {
    fn convert(controller: AccountId) -> Option<AccountId> {
        Some(controller)
    }
}

/// Means for interacting with a specialized version of the `session` trait.
///
/// This is needed because `Staking` sets the `ValidatorIdOf` of the `pallet_session::Config`
pub trait SessionInterface<AccountId>: frame_system::Config {
    /// Disable a given validator by stash ID.
    ///
    /// Returns `true` if new era should be forced at the end of this session.
    /// This allows preventing a situation where there is too many validators
    /// disabled and block production stalls.
    fn disable_validator(validator: &AccountId) -> Result<bool, ()>;
    /// Get the validators from session.
    fn validators() -> Vec<AccountId>;
    /// Prune historical session tries up to but not including the given index.
    fn prune_historical_up_to(up_to: SessionIndex);
}

impl<T: Config> SessionInterface<<T as frame_system::Config>::AccountId> for T
where
    T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
    T: pallet_session::historical::Config<
        FullIdentification = <T as frame_system::Config>::AccountId,
        FullIdentificationOf = SimpleValidatorIdConverter<<T as frame_system::Config>::AccountId>,
    >,
    T::SessionHandler: pallet_session::SessionHandler<<T as frame_system::Config>::AccountId>,
    T::SessionManager: pallet_session::SessionManager<<T as frame_system::Config>::AccountId>,
    T::ValidatorIdOf: Convert<
        <T as frame_system::Config>::AccountId,
        Option<<T as frame_system::Config>::AccountId>,
    >,
{
    fn disable_validator(validator: &<T as frame_system::Config>::AccountId) -> Result<bool, ()> {
        <pallet_session::Module<T>>::disable(validator)
    }

    fn validators() -> Vec<<T as frame_system::Config>::AccountId> {
        <pallet_session::Module<T>>::validators()
    }

    fn prune_historical_up_to(up_to: SessionIndex) {
        <pallet_session::historical::Module<T>>::prune_up_to(up_to);
    }
}
