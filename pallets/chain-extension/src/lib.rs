#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::{
    decl_module, decl_storage,
    traits::{Get, Randomness},
    weights::Weight,
    Parameter,
};
use jupiter_chain_extension::JupiterExt;
use jupiter_primitives::Hash;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, Result, RetVal, SysConfig, UncheckedFrom,
};
use pallet_session::SessionManager;

use sp_consensus_vrf::schnorrkel;
use sp_runtime::traits::Member;
use sp_staking::SessionIndex;
use sp_std::prelude::*;

#[derive(Debug, PartialEq, Encode)]
pub struct BabeRandomness {
    pub epoch: u64,
    pub start_slot: u64,
    pub duration: u64,
    pub randomness: Hash,
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
    pub fn current_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::current_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: Hash::from(epoch.randomness),
        }
    }

    pub fn next_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::next_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: Hash::from(epoch.randomness),
        }
    }

    pub fn randomness_of(epoch: u64) -> Hash {
        Hash::from(<HistoricalRandomness>::get(&epoch))
    }

    pub fn random(subject: &[u8]) -> T::Hash {
        <pallet_babe::Module<T>>::random(subject)
    }
}

impl<T: Config> ChainExtension for Module<T> {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal>
    where
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        match func_id {
            // 0x0000ffff - 0x01000000
            // 0x00010000 - 0x0001ffff => std runtime
            // current epoch randomness
            0x00010000 => {
                let mut env = env.buf_in_buf_out();
                // reference seal_random
                let gas = (166_160_000 as Weight)
                    // Standard Error: 237_000
                    .saturating_add((594_474_000 as Weight).saturating_mul(1 as Weight))
                    .saturating_add(<E::T as SysConfig>::DbWeight::get().reads(6 as Weight));
                env.charge_weight(gas)?;

                let cur_epoch = Self::current_epoch();
                env.write(&cur_epoch.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // next epoch randomness
            0x00010001 => {
                let mut env = env.buf_in_buf_out();
                // reference seal_random
                let gas = (166_160_000 as Weight)
                    // Standard Error: 237_000
                    .saturating_add((594_474_000 as Weight).saturating_mul(1 as Weight))
                    .saturating_add(<E::T as SysConfig>::DbWeight::get().reads(6 as Weight));
                env.charge_weight(gas)?;

                let next_epoch = Self::next_epoch();
                env.write(&next_epoch.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // query historical randomness with specified epoch
            0x00010002 => {
                let mut env = env.buf_in_buf_out();
                // reference seal_random
                let gas = (166_160_000 as Weight)
                    // Standard Error: 237_000
                    .saturating_add((594_474_000 as Weight).saturating_mul(1 as Weight))
                    .saturating_add(<E::T as SysConfig>::DbWeight::get().reads(6 as Weight));
                env.charge_weight(gas)?;

                let input: u64 = env.read_as()?;
                let randomness = Self::randomness_of(input);
                env.write(&randomness.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // random with subject input
            0x00010003 => {
                let mut env = env.buf_in_buf_out();
                // reference seal_random
                let gas = (166_160_000 as Weight)
                    // Standard Error: 237_000
                    .saturating_add((594_474_000 as Weight).saturating_mul(1 as Weight))
                    .saturating_add(<E::T as SysConfig>::DbWeight::get().reads(6 as Weight));
                env.charge_weight(gas)?;

                let input: Vec<u8> = env.read_as()?;
                let randomness = Self::random(input.as_slice());
                env.write(&randomness.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            _ => JupiterExt::call(func_id, env),
        }
    }

    fn enabled() -> bool {
        true
    }
}
