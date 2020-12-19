//! Jupiter Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

/// The chain Extension of Jupiter
pub struct JupiterExt;

impl ChainExtension for JupiterExt {
    fn call<E: Ext>(
        func_id: u32,
        mut env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        sp_std::if_std! {}
        // env.charge_weight(1_000_000_000_u64)?;
        let mut env = env.buf_in_buf_out();
        let input: Vec<u8> = env.read_as()?;
        sp_std::if_std! {
            println!("fn_id: {}, input_len: {:?}", func_id, input.len());
        }
        let output = curve::call(func_id, &input).map_err(|e| {
            sp_std::if_std! {
                println!("{:?}", e);
            }
            DispatchError::Other("Call chain extension failed")
        })?;
        env.write(&output, false, None)?;
        Ok(RetVal::Converging(func_id))
    }

    fn enabled() -> bool {
        true
    }
}
