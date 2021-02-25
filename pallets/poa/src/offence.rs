use frame_support::{debug::warn, traits::Get, weights::Weight};
use sp_runtime::{traits::Convert, Perbill};
use sp_staking::{
    offence::{OffenceDetails, OnOffenceHandler},
    SessionIndex,
};

use crate::{
    session::SimpleValidatorIdConverter, slashing, Config, EarliestUnappliedSlash, Pallet,
    UnappliedSlashes, LOG_TARGET,
};

/// This is intended to be used with `FilterHistoricalOffences`.
impl<T: Config>
    OnOffenceHandler<T::AccountId, pallet_session::historical::IdentificationTuple<T>, Weight>
    for Pallet<T>
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
    fn on_offence(
        offenders: &[OffenceDetails<
            T::AccountId,
            pallet_session::historical::IdentificationTuple<T>,
        >],
        slash_fraction: &[Perbill],
        slash_session: SessionIndex,
    ) -> Result<Weight, ()> {
        if !Self::can_report() {
            return Err(());
        }

        // let reward_proportion = SlashRewardFraction::get();
        let mut consumed_weight: Weight = 0;
        let mut add_db_reads_writes = |reads, writes| {
            consumed_weight += T::DbWeight::get().reads_writes(reads, writes);
        };

        let active_era = {
            let active_era = Self::active_era();
            add_db_reads_writes(1, 0);
            if active_era.is_none() {
                // this offence need not be re-submitted.
                return Ok(consumed_weight);
            }
            active_era
                .expect("value checked not to be `None`; qed")
                .index
        };
        let active_era_start_session_index = Self::eras_start_session_index(active_era)
            .unwrap_or_else(|| {
                frame_support::print("Error: start_session_index must be set for current_era");
                0
            });
        add_db_reads_writes(1, 0);

        // fast path for active-era report - most likely.
        // `slash_session` cannot be in a future active era. It must be in `active_era` or before.
        let slash_era = if slash_session >= active_era_start_session_index {
            active_era
        } else {
            // TODO may change this
            // before current Era. defensive - should be filtered out.
            return Ok(consumed_weight);
        };

        EarliestUnappliedSlash::<T>::mutate(|earliest| {
            if earliest.is_none() {
                *earliest = Some(active_era)
            }
        });
        add_db_reads_writes(1, 1);

        let slash_defer_duration = T::SlashDeferDuration::get();

        let invulnerables = Self::invulnerables();
        add_db_reads_writes(1, 0);

        // TODO need to limit minimal validators
        for (details, _) in offenders.iter().zip(slash_fraction) {
            let (stash, _) = &details.offender;

            // Skip if the validator is invulnerable.
            if invulnerables.contains(stash) {
                warn!(
                    target: LOG_TARGET,
                    "receive offence, but in invulnerables list:{:?}", stash
                );
                continue;
            }

            let unapplied = slashing::set_to_waiting::<T>(slashing::SlashParams {
                validator: stash,
                slash_era,
                now: active_era,
            });

            if let Some(mut unapplied) = unapplied {
                unapplied.reporters = details.reporters.clone();

                if slash_defer_duration == 0 {
                    // apply right away.
                    slashing::apply_slash::<T>(unapplied.validator.clone());
                    add_db_reads_writes(1, 1);
                } else {
                    // defer to end of some `slash_defer_duration` from now.
                    UnappliedSlashes::<T>::mutate(active_era, move |for_later| {
                        for_later.push(unapplied)
                    });
                    add_db_reads_writes(1, 1);
                }
            } else {
                add_db_reads_writes(1, 1);
            }
        }

        Ok(consumed_weight)
    }

    fn can_report() -> bool {
        // TODO current set false until test all case
        // true
        false
    }
}
