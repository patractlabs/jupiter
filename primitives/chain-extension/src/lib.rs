//! Jupiter Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use parity_scale_codec::Encode;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

/// The chain Extension of Jupiter
pub struct JupiterExt;

impl ChainExtension for JupiterExt {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        let mut env = env.buf_in_buf_out();
        env.charge_weight(1_000_000_000_u64)?;

        // The memory of the vm stores buf in scale-codec
        let input: Vec<u8> = env.read_as()?;
        let raw_output: Vec<u8> = curve::call(func_id, &input)
            .map_err(|_| DispatchError::Other("Call chain extension failed"))?;

        // Encode back to the memory
        let output: Vec<u8> = raw_output.encode();
        env.write(&output, false, None)?;
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
