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

        frame_support::debug::native::trace!(
            target: "runtime",
            "[ChainExtension]|call|func_id:{:}|input:{:}",
            func_id,
            hex::encode(&input)
        );

        #[allow(unused_assignments)]
        let mut raw_output: Vec<u8> = Vec::with_capacity(0);
        #[cfg(feature = "native-support")]
        {
            raw_output = jupiter_io::pairing::call(func_id, &input).ok_or(DispatchError::Other(
                "ChainExtension failed to call native `jupiter_io::pairing`",
            ))?;
        }
        #[cfg(not(feature = "native-support"))]
        {
            raw_output = curve::call(func_id, &input).map_err(|e| {
                frame_support::debug::error!(
                    "call zkp lib `curve::call` meet an error|func_id:{:}|err:{:?}",
                    func_id,
                    e
                );
                DispatchError::Other("ChainExtension failed to call `curve::call`")
            })?;
        }

        // Encode back to the memory
        let output: Vec<u8> = raw_output.encode();
        env.write(&output, false, None)?;
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
