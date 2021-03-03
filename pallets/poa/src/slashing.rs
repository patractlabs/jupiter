use sp_std::prelude::*;

use crate::session::SessionInterface;
use crate::{Config, EraIndex, Event, Pallet, UnappliedSlash};

pub(crate) fn apply_slash<T: Config>(unapplied_slash: T::AccountId) {
    Pallet::<T>::remove_from_authorities(&unapplied_slash);
}

/// Parameters for performing a slash.
#[derive(Clone)]
pub(crate) struct SlashParams<'a, T: 'a + Config> {
    /// The validator account being slashed.
    pub(crate) validator: &'a T::AccountId,
    /// The era where the offence occurred.
    pub(crate) slash_era: EraIndex,
    /// The current era.
    pub(crate) now: EraIndex,
}
pub(crate) fn set_to_waiting<T: Config>(
    params: SlashParams<T>,
) -> Option<UnappliedSlash<T::AccountId>> {
    let SlashParams {
        validator,
        slash_era: _,
        now: _,
    } = params;

    Pallet::<T>::set_waiting(validator)?;
    Pallet::<T>::deposit_event(Event::<T>::Offence(validator.clone()));

    if T::SessionInterface::disable_validator(validator).unwrap_or(false) {
        // force a new era, to select a new validator set
        Pallet::<T>::ensure_new_era();
    }
    Some(UnappliedSlash {
        validator: validator.clone(),
        reporters: Vec::new(),
    })
}
