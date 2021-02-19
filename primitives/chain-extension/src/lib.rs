//! Jupiter Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use parity_scale_codec::Encode;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

use frame_support::debug::{error, native};

/// The chain Extension of Patract
pub struct PatractExt;

impl<C: pallet_contracts::Config> ChainExtension<C> for PatractExt {
    fn call<E>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
        E: Ext<T = C>,
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        let mut env = env.buf_in_buf_out();

        // func_id refer to https://github.com/patractlabs/PIPs/blob/main/PIPs/pip-100.md
        match func_id {
            // 0x01000000-0x010000ff Patract ZKP Support
            0x01000000..=0x010000ff => {
                // The memory of the vm stores buf in scale-codec
                let input: Vec<u8> = env.read_as()?;
                // currently only support [PIP-101](https://github.com/patractlabs/PIPs/blob/main/PIPs/pip-101.md)
                // TODO just charge weight in a simple way. ADD/MUL is less then SHA256's weight
                // and Paring is more than SHA256's weight. Change this part with benchmark result in future.
                let simple_weight = match func_id & 0x01 {
                    0 => 100_000,   // add, In ethereum: 500
                    1 => 8_000_000, // 80x then add, In ethereum: 40000
                    2 => {
                        // paring.
                        // In ethereum:
                        // Pairing ：80 000 * k + 100 000, where k is the number of points or, equivalently, the length of the input divided by 192
                        let k = if input.len() > 194 {
                            input.len() as u64 / 194
                        } else {
                            1
                        };
                        16_000_000 * k
                    }
                    _ => {
                        error!("[PIP-101]call an unregistered `func_id` in Patract ZKP field, func_id:{:}", func_id);
                        return Err(DispatchError::Other("Unimplemented Patract ZKP func_id"));
                    }
                };
                env.charge_weight(simple_weight)?;

                native::trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}|charge-weight:{:}|input:{:}",
                    func_id,
                    simple_weight,
                    hex::encode(&input)
                );

                #[allow(unused_assignments)]
                let mut raw_output: Vec<u8> = Vec::with_capacity(0);
                #[cfg(feature = "native-support")]
                {
                    raw_output =
                        patract_io::pairing::call(func_id, &input).ok_or(DispatchError::Other(
                            "ChainExtension failed to call native `patract_io::pairing`",
                        ))?;
                }
                #[cfg(not(feature = "native-support"))]
                {
                    raw_output = curve::call(func_id, &input).map_err(|e| {
                        error!(
                            "call zkp lib `curve::call` meet an error|func_id:{:}|err:{:?}",
                            func_id, e
                        );
                        DispatchError::Other("ChainExtension failed to call `curve::call`")
                    })?;
                }

                // Encode back to the memory
                let output: Vec<u8> = raw_output.encode();
                env.write(&output, false, None)?;
            }
            _ => {
                error!("call an unregistered `func_id`, func_id:{:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }

        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
