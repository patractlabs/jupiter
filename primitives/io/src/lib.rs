//! Jupiter primitives - IO Module
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime_interface::runtime_interface;

/// ZK-SNARKs runtime interface
#[runtime_interface]
pub trait ZkSnarks {
    /// altbn_128 add
    fn altbn_128_add(input: &[u8]) -> Option<[u8; 64]> {
        megaclite::altbn_128::add(input).ok()
    }

    /// altbn_128 mul
    fn altbn_128_mul(input: &[u8]) -> Option<[u8; 64]> {
        megaclite::altbn_128::mul(input).ok()
    }

    /// altbn_128 pairing
    fn altbn_128_pairing(input: &[u8]) -> Option<bool> {
        megaclite::altbn_128::pairing(input).ok()
    }

    /// bls12_381 add
    fn bls12_381_add(input: &[u8]) -> Option<[u8; 96]> {
        megaclite::bls12_381::add(input).ok()
    }

    /// bls12_381 mul
    fn bls12_381_mul(input: &[u8]) -> Option<[u8; 96]> {
        megaclite::bls12_381::mul(input).ok()
    }

    /// bls12_381 pairing
    fn bls12_381_pairing(input: &[u8]) -> Option<bool> {
        megaclite::bls12_381::pairing(input).ok()
    }
}
