#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::upper_case_acronyms)]

use codec::{Decode, Encode, FullCodec, HasCompact};
use frame_support::pallet_prelude::{DispatchClass, Pays, Weight};
use jupiter_primitives::{
    CurrencyId,
};
use sp_core::H160;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
    transaction_validity::TransactionValidityError,
    DispatchError, DispatchResult, FixedU128, RuntimeDebug,
};
use sp_std::{
    cmp::{Eq, PartialEq},
    fmt::Debug,
    prelude::*,
};

pub trait TransactionPayment<AccountId, Balance, NegativeImbalance> {
    fn reserve_fee(who: &AccountId, weight: Weight) -> Result<Balance, DispatchError>;
    fn unreserve_fee(who: &AccountId, fee: Balance);
    fn unreserve_and_charge_fee(
        who: &AccountId,
        weight: Weight,
    ) -> Result<(Balance, NegativeImbalance), TransactionValidityError>;
    fn refund_fee(who: &AccountId, weight: Weight, payed: NegativeImbalance) -> Result<(), TransactionValidityError>;
    fn charge_fee(
        who: &AccountId,
        len: u32,
        weight: Weight,
        tip: Balance,
        pays_fee: Pays,
        class: DispatchClass,
    ) -> Result<(), TransactionValidityError>;
}

#[cfg(feature = "std")]
use frame_support::traits::Imbalance;
#[cfg(feature = "std")]
impl<AccountId, Balance: Default + Copy, NegativeImbalance: Imbalance<Balance>>
TransactionPayment<AccountId, Balance, NegativeImbalance> for ()
{
    fn reserve_fee(_who: &AccountId, _weight: Weight) -> Result<Balance, DispatchError> {
        Ok(Default::default())
    }

    fn unreserve_fee(_who: &AccountId, _fee: Balance) {}

    fn unreserve_and_charge_fee(
        _who: &AccountId,
        _weight: Weight,
    ) -> Result<(Balance, NegativeImbalance), TransactionValidityError> {
        Ok((Default::default(), Imbalance::zero()))
    }

    fn refund_fee(
        _who: &AccountId,
        _weight: Weight,
        _payed: NegativeImbalance,
    ) -> Result<(), TransactionValidityError> {
        Ok(())
    }

    fn charge_fee(
        _who: &AccountId,
        _len: u32,
        _weight: Weight,
        _tip: Balance,
        _pays_fee: Pays,
        _class: DispatchClass,
    ) -> Result<(), TransactionValidityError> {
        Ok(())
    }
}
