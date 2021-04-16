#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::{decl_module, decl_storage, traits::Randomness as RandomnessT, Parameter};

use pallet_session::SessionManager;

use sp_consensus_vrf::schnorrkel;
use sp_runtime::traits::Member;
use sp_staking::SessionIndex;
use sp_std::prelude::*;

// SBP-M1 review: I would like to better understand the rationale and design decisions
// that guided for implementing the NoteHistoricalRandomness runtime logic.
// I think this should be well documented, and that any security-related caveats should
// be clearly highlighted for contract developers (as the randomness is exposed to them
// via a contract primitive & chain extension).
//
// I want to point out that in the BABE pallet, there is specific warning about exposing
// or using BABE consensus randomness from higer-level code (e.g. contracts) for things
// like gambling (or any application-specific randomness), as malicious attackers could
// take advantage of the fact that previous & current epoch randomness are publicly available.
//
// See https://github.com/paritytech/substrate/blob/982df173190685bb18b7ed46b860e36387b9a7a9/frame/babe/src/lib.rs#L198
// ---------
// The epoch randomness for the *current* epoch.
//
// # Security
//
// This MUST NOT be used for gambling, as it can be influenced by a
// malicious validator in the short term. It MAY be used in many
// cryptographic protocols, however, so long as one remembers that this
// (like everything else on-chain) it is public.
// ---------
//
// FYI, some changes have happened on Substrate's master branch re.
// BABE randomness and how it is exposed & used by some pallets, notably the contracts pallet.
// 
// You might find the following issues & PRs relevant.
// - https://github.com/paritytech/substrate/pull/8180
// - https://github.com/paritytech/substrate/commit/b24c43af1a77168feed445136ea4569f1d132c3a#diff-445c7596b2aa45b11dd9e9e3013e3747b5c5dfec7ba012c690252fa41ebb8678
// - https://github.com/paritytech/substrate/pull/8180/commits/1ce1e9d7b020208f3bb6f8318ac24bea90519b83
// - https://github.com/paritytech/substrate/issues/8297
// - https://github.com/paritytech/substrate/pull/8329

#[derive(Debug, PartialEq, Encode)]
pub struct BabeRandomness {
    pub epoch: u64,
    pub start_slot: u64,
    pub duration: u64,
    pub randomness: schnorrkel::Randomness,
}

pub trait Config: pallet_babe::Config + frame_system::Config {
    /// A stable ID for a validator.
    type ValidatorId: Member + Parameter;
}

decl_storage! {
    trait Store for Module<T: Config> as RandomnessExt {
        /// Historical randomness for each epoch.
        HistoricalRandomness get(fn historical_randomness): map hasher(twox_64_concat) u64 => schnorrkel::Randomness;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {}
}

/// An `SessionManager` implementation that wraps an inner `I` and also
/// sets the historical randomness of the ending session.
pub struct NoteHistoricalRandomness<T, I>(sp_std::marker::PhantomData<(T, I)>);

impl<T: Config, I> SessionManager<T::ValidatorId> for NoteHistoricalRandomness<T, I>
where
    I: SessionManager<T::ValidatorId>,
{
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        <I as SessionManager<_>>::new_session(new_index)
    }

    fn end_session(end_index: SessionIndex) {
        <I as SessionManager<_>>::end_session(end_index);
        let epoch = <pallet_babe::Module<T>>::current_epoch();
        <HistoricalRandomness>::insert(epoch.epoch_index, epoch.randomness);
    }

    fn start_session(start_index: SessionIndex) {
        <I as SessionManager<_>>::start_session(start_index)
    }
}

impl<T: Config> Module<T> {
    // Return babe randomness info for current epoch
    pub fn current_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::current_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: epoch.randomness,
        }
    }

    // Return babe randomness info for next epoch
    pub fn next_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::next_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: epoch.randomness,
        }
    }

    /// Return babe randomness for historical epoch
    pub fn randomness_of(epoch: u64) -> schnorrkel::Randomness {
        <HistoricalRandomness>::get(&epoch)
    }

    /// Return randomness with provider subject
    pub fn random(subject: &[u8]) -> T::Hash {
        <pallet_babe::Module<T>>::random(subject)
    }
}
