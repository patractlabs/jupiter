//! Jupiter primitives - IO Module
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

pub mod groth16;

/// Pairing runtime interface
#[runtime_interface]
pub trait Pairing {
    /// Vector Addition
    fn add(curve_id: i32, input: &[u8]) -> Option<Vec<u8>> {
        curve::add(curve_id, input).ok()
    }

    /// Scalar Multiplication
    fn mul(curve_id: i32, input: &[u8]) -> Option<Vec<u8>> {
        curve::mul(curve_id, input).ok()
    }

    /// Pairing
    fn pairing(curve_id: i32, input: &[u8]) -> Option<bool> {
        curve::pairing(curve_id, input).ok()
    }
}
