#![cfg_attr(not(feature = "std"), no_std)]

pub mod constants;
pub mod impls;
pub mod weights;

/// The type used for currency conversion.
///
/// This must only be used as long as the balance type is u128.
pub type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
