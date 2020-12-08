//! Jupiter primitives - IO Module
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime_interface::runtime_interface;

pub mod groth16;

/// Pairing runtime interface
#[runtime_interface]
pub trait Pairing {
    /// Vector Addition
    fn add(curve_id: i32, input: &[u8]) -> Option<Vec<u8>> {
        Some(match curve_id {
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
        }?)
    }

    /// Scalar Multiplication
    fn mul(curve_id: i32, input: &[u8]) -> Option<Vec<u8>> {
        Some(match curve_id {
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
        }?)
    }

    /// Pairing
    fn pairing(curve_id: i32, input: &[u8]) -> Option<bool> {
        Some(match curve_id {
            0x2a => {
                <megaclite::curves::Bls12_377 as megaclite::CurveBasicOperations>::pairings(input)
                    .ok()
            }
            0x2b => {
                <megaclite::curves::Bls12_381 as megaclite::CurveBasicOperations>::pairings(input)
                    .ok()
            }
            0x2c => {
                <megaclite::curves::Bn254 as megaclite::CurveBasicOperations>::pairings(input).ok()
            }
            0x2d => {
                <megaclite::curves::BW6_761 as megaclite::CurveBasicOperations>::pairings(input)
                    .ok()
            }
            _ => None,
        }?)
    }
}
