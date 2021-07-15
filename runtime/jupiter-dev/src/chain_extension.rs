use codec::Encode;

use frame_support::{traits::Get, weights::Weight, traits::Randomness};

use sp_core::H256;
use sp_runtime::traits::Hash;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, Result, RetVal, SysConfig, UncheckedFrom,
};
use pallet_randomness_provider::BabeRandomness;

use jupiter_chain_extension::JupiterExt;

use crate::Runtime;
use frame_support::{
    dispatch::DispatchError,
    log::{debug, error, info, trace, warn},
};

pub struct DevExtension<C>(PhantomData<C>);

impl<C: pallet_contracts::Config> ChainExtension<C> for DevExtension<C> {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal>
    where
        E: Ext<T = C>,
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        let randomness_gas = || {
            // reference seal_random
            (166_160_000 as Weight)
                // Standard Error: 237_000
                .saturating_add((594_474_000 as Weight).saturating_mul(1 as Weight))
                .saturating_add(<E::T as SysConfig>::DbWeight::get().reads(6 as Weight))
        };

        match func_id {
            // 0x0000ffff - 0x01000000
            // 0x00010000 - 0x0001ffff => std runtime
            // current epoch randomness
            0x00010000 => {
                let mut env = env.buf_in_buf_out();

                env.charge_weight(randomness_gas())?;
                // mock for Jupiter-dev
                let cur_epoch: u64 = 1;
                env.write(&cur_epoch.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // next epoch randomness
            0x00010001 => {
                let mut env = env.buf_in_buf_out();

                env.charge_weight(randomness_gas())?;

                // mock for Jupiter-dev
                let next_epoch = BabeRandomness {
                    epoch: 1,
                    start_slot: 1,
                    duration: 1,
                    randomness: Default::default(),
                };
                env.write(&next_epoch.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // query historical randomness with specified epoch
            0x00010002 => {
                let mut env = env.buf_in_buf_out();

                env.charge_weight(randomness_gas())?;

                let _: u64 = env.read_as()?;
                // mock for Jupiter-dev
                let randomness: H256 = Default::default();
                env.write(&randomness.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // random with subject input
            0x00010003 => {
                let mut env = env.buf_in_buf_out();

                env.charge_weight(randomness_gas())?;

                let len = env.in_len();
                let input: Vec<u8> = env.read_as_unbounded(len)?;

                let randomness =
                    <Runtime as frame_system::Config>::Hashing::hash_of(&input.as_slice());
                env.write(&randomness.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            1101 => {
                let mut env = env.buf_in_buf_out();
                let random_seed = crate::RandomnessCollectiveFlip::random_seed().0;
                let random_slice = random_seed.encode();
                trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}, rand-seed:{:?}, rand-slice:{:?}",
                    func_id, random_seed, random_slice
                );
                env.write(&random_slice, false, None).map_err(|_| {
                    DispatchError::Other("ChainExtension failed to call random")
                })?;
                Ok(RetVal::Converging(0))
            }
            1102 => {
                let mut env = env.buf_in_buf_out();
                let len = env.in_len();
                let input: Vec<u8> = env.read_as_unbounded(len)?;
                let randomness = <Runtime as frame_system::Config>::Hashing::hash_of(&input.as_slice());
                trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}, len:{:?}, vec:{:?}, encode:{:?}",
                    func_id, len, input, &randomness.encode()
                );
                env.write(&randomness.encode(), false, None)?;

                // let random_seed = crate::RandomnessCollectiveFlip::random_seed().0;
                // let random_slice = random_seed.encode();
                // env.write(&random_slice, false, None).map_err(|_| {
                //     DispatchError::Other("ChainExtension failed to call random")
                // })?;
                Ok(RetVal::Converging(0))
            }
            0xfeffff00 => {
                runtime_log::logger_ext!(func_id, env);
                // env.write(&randomness.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }

            _ => JupiterExt::call(func_id, env),
        }
    }

    fn enabled() -> bool {
        true
    }
}
