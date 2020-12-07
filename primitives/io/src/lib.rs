//! Jupiter primitives - IO Module
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime_interface::runtime_interface;

/// ZK-SNARKs runtime interface
#[runtime_interface]
pub trait ZkSnarks {
    /// Vector Addition
    fn add(fn_id: i32, input: &[u8], output: &mut [u8]) -> Option<()> {
        let res = match fn_id {
            0x2a => {
                <megaclite::curves::Bls12_377 as megaclite::CurveBasicOperations>::add(input).ok()
            }
            0x2b => {
                <megaclite::curves::Bls12_381 as megaclite::CurveBasicOperations>::add(input).ok()
            }
            0x2c => <megaclite::curves::Bn254 as megaclite::CurveBasicOperations>::add(input).ok(),
            0x2d => {
                <megaclite::curves::BW6_761 as megaclite::CurveBasicOperations>::add(input).ok()
            }
            _ => None,
        }?;

        output.copy_from_slice(&res);
        Some(())
    }

    /// Scalar Multiplication
    fn mul(fn_id: i32, input: &[u8], output: &mut [u8]) -> Option<()> {
        let res = match fn_id {
            0x2a => {
                <megaclite::curves::Bls12_377 as megaclite::CurveBasicOperations>::mul(input).ok()
            }
            0x2b => {
                <megaclite::curves::Bls12_381 as megaclite::CurveBasicOperations>::mul(input).ok()
            }
            0x2c => <megaclite::curves::Bn254 as megaclite::CurveBasicOperations>::mul(input).ok(),
            0x2d => {
                <megaclite::curves::BW6_761 as megaclite::CurveBasicOperations>::mul(input).ok()
            }
            _ => None,
        }?;

        output.copy_from_slice(&res);
        Some(())
    }

    /// Scalar Multiplication
    fn pairing(fn_id: i32, input: &[u8]) -> Option<bool> {
        let res = match fn_id {
            0x2a => {
                <megaclite::curves::Bls12_377 as megaclite::CurveBasicOperations>::pairing(input)
                    .ok()
            }
            0x2b => {
                <megaclite::curves::Bls12_381 as megaclite::CurveBasicOperations>::pairing(input)
                    .ok()
            }
            0x2c => {
                <megaclite::curves::Bn254 as megaclite::CurveBasicOperations>::pairing(input).ok()
            }
            0x2d => {
                <megaclite::curves::BW6_761 as megaclite::CurveBasicOperations>::pairing(input).ok()
            }
            _ => None,
        }?;

        Some(res)
    }
}
