//! Jupiter Chain Extension
use pallet_contracts::{
    chain_extension::{
        ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
    },
    Error,
};
use sp_runtime::DispatchError;

/// Convert bool to bytes, 0 for true, 1 for false
fn b2b(b: bool) -> Vec<u8> {
    if b {
        Vec::from([0])
    } else {
        Vec::from([1])
    }
}

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
        if let Some(output) = match func_id {
            // Use runtime interfaces
            0x2a => jupiter_io::pairing::add(0x2a, input),
            0x2b => jupiter_io::pairing::add(0x2b, input),
            0x2c => jupiter_io::pairing::add(0x2c, input),
            0x2d => jupiter_io::pairing::add(0x2d, input),
            0x3a => jupiter_io::pairing::mul(0x2a, input),
            0x3b => jupiter_io::pairing::mul(0x2b, input),
            0x3c => jupiter_io::pairing::mul(0x2c, input),
            0x3d => jupiter_io::pairing::mul(0x2d, input),
            0x4a => jupiter_io::pairing::pairing(0x2a, input).map(|b| b2b(b)),
            0x4b => jupiter_io::pairing::pairing(0x2b, input).map(|b| b2b(b)),
            0x4c => jupiter_io::pairing::pairing(0x2c, input).map(|b| b2b(b)),
            0x4d => jupiter_io::pairing::pairing(0x2d, input).map(|b| b2b(b)),
            // Use megaclite
            0x5a => curve::add(0x2a, input).ok(),
            0x5b => curve::add(0x2b, input).ok(),
            0x5c => curve::add(0x2c, input).ok(),
            0x5d => curve::add(0x2d, input).ok(),
            0x6a => curve::mul(0x2a, input).ok(),
            0x6b => curve::mul(0x2b, input).ok(),
            0x6c => curve::mul(0x2c, input).ok(),
            0x6d => curve::mul(0x2d, input).ok(),
            0x7a => curve::pairing(0x2a, input).map(|b| b2b(b)).ok(),
            0x7b => curve::pairing(0x2b, input).map(|b| b2b(b)).ok(),
            0x7c => curve::pairing(0x2c, input).map(|b| b2b(b)).ok(),
            0x7d => curve::pairing(0x2d, input).map(|b| b2b(b)).ok(),
            _ => return Err(Error::<E::T>::NoChainExtension.into()),
        } {
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
