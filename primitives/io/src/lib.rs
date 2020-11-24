//! Jupiter primitives - IO Module
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
use megaclite::{altbn_128, bls12_381};
use sp_runtime_interface::runtime_interface;

/// ZK-SNARKs runtime interface
#[runtime_interface]
pub trait ZkSnarks {
    /// altbn_128 add
    fn altbn_128_add(input: &[u8]) -> [u8; 64] {
        if let Ok(res) = altbn_128::add(input) {
            res
        } else {
            [0; 64]
        }
    }

    /// altbn_128 mul
    fn altbn_128_mul(input: &[u8]) -> [u8; 64] {
        if let Ok(res) = altbn_128::mul(input) {
            res
        } else {
            [0; 64]
        }
    }

    /// altbn_128 pairing
    fn altbn_128_pairing(input: &[u8]) -> bool {
        if let Ok(res) = altbn_128::pairing(input) {
            res
        } else {
            false
        }
    }

    /// bls12_381 add
    fn bls12_381_add(input: &[u8]) -> [u8; 96] {
        if let Ok(res) = bls12_381::add(input) {
            res
        } else {
            [0; 96]
        }
    }

    /// bls12_381 mul
    fn bls12_381_mul(input: &[u8]) -> [u8; 96] {
        if let Ok(res) = bls12_381::mul(input) {
            res
        } else {
            [0; 96]
        }
    }

    /// bls12_381 pairing
    fn bls12_381_pairing(input: &[u8]) -> bool {
        if let Ok(res) = bls12_381::pairing(input) {
            res
        } else {
            false
        }
    }
}
