//! Jupiter Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::DispatchError;
use sp_std::{str, vec::Vec};

use frame_support::debug::{error, native};

/// The chain Extension of Jupiter
pub struct JupiterExt;

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct LogRecord {
    level: u32,
    target: Vec<u8>,
    args: Vec<u8>,
}

impl ChainExtension for JupiterExt {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
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
                        jupiter_io::pairing::call(func_id, &input).ok_or(DispatchError::Other(
                            "ChainExtension failed to call native `jupiter_io::pairing`",
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
            // 0xfeffff00-0xfeffffff reserved for pallet-contracts log and print system
            // 0xfeffff00 => ink-log
            0xfeffff00 => {
                // The memory of the vm stores buf in scale-codec
                let input: LogRecord = env.read_as()?;
                let target = str::from_utf8(input.target.as_slice()).unwrap();
                let args = str::from_utf8(input.args.as_slice()).unwrap();

                match input.level {
                    1 => {
                        native::error!(target: target, "❌ {}", args);
                    }
                    2 => {
                        native::warn!(target: target, "⚠️  {}", args);
                    }
                    3 => {
                        native::info!(target: target, "❤️  {}", args);
                    }
                    4 => {
                        native::debug!(target: target, "📋  {}", args);
                    }
                    5 => {
                        native::trace!(target: target, "🏷  {}", args);
                    }
                    _ => (),
                }
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
