use codec::Encode;

use frame_support::{traits::Get, weights::Weight};

use sp_core::H256;
use sp_std::vec::Vec;

use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, Result, RetVal, SysConfig, UncheckedFrom,
};

use jupiter_chain_extension::JupiterExt;

use crate::RandomnessProvider;

pub struct JupiterExtension;

impl ChainExtension for JupiterExtension {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal>
    where
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

                let cur_epoch = RandomnessProvider::current_epoch();
                env.write(&cur_epoch.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // next epoch randomness
            0x00010001 => {
                let mut env = env.buf_in_buf_out();

                env.charge_weight(randomness_gas())?;

                let next_epoch = RandomnessProvider::next_epoch();
                env.write(&next_epoch.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // query historical randomness with specified epoch
            0x00010002 => {
                let mut env = env.buf_in_buf_out();

                env.charge_weight(randomness_gas())?;

                let input: u64 = env.read_as()?;

                let randomness = H256::from(RandomnessProvider::randomness_of(input));
                env.write(&randomness.encode(), false, None)?;
                Ok(RetVal::Converging(0))
            }
            // random with subject input
            0x00010003 => {
                let mut env = env.buf_in_buf_out();

                env.charge_weight(randomness_gas())?;

                let input: Vec<u8> = env.read_as()?;
                let randomness = RandomnessProvider::random(input.as_slice());
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
