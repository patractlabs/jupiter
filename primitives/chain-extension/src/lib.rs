//! Jupiter Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
use pallet_contracts::{
    chain_extension::{
        ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
    },
    Error,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

/// The chain Extension of Jupiter
pub struct JupiterExt;

impl ChainExtension for JupiterExt {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        let mut dimmy: Vec<u8> = Vec::new();
        let mut input = dimmy.as_mut_slice();
        let mut env = env.buf_in_buf_out();
        env.read_into(&mut &mut input)?;

        #[allow(unused_assignments)]
        let mut result: Option<Vec<u8>> = None;
        #[cfg(feature = "native-support")]
        {
            result = jupiter_io::pairing::call(func_id, input);
        }
        #[cfg(not(feature = "native-support"))]
        {
            result = curve::call(func_id, input).ok();
        }

        if let Some(output) = result {
            env.write(&output, false, None)?;
            Ok(RetVal::Converging(0))
        } else {
            Err(Error::<E::T>::InvalidFunctionId.into())
        }
    }

    fn enabled() -> bool {
        true
    }
}
