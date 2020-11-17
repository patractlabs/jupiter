//! Jupiter primitives - IO Module
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime_interface::runtime_interface;

/// ZK-SNARKs runtime interface
#[runtime_interface]
pub trait ZkSnarks {
    /// altbn_128 add
    // /// ```
    // /// assert_eq!(jupiter_io::zk_snarks::altbn_128_add(1, 1), 2);
    // /// ```
    fn altbn_128_add(_g1: &[u8], _g2: &[u8]) -> [u8; 64] {
        [0; 64]
    }

    /// altbn_128 mul
    // /// ```
    // /// assert_eq!(jupiter_io::zk_snarks::altbn_128_mul(2, 2), 4);
    // /// ```
    fn altbn_128_mul(_input: &[u8], _scalar: u64) -> [u8; 64] {
        [1; 64]
    }

    /// altbn_128 pairing
    // /// ```
    // /// assert_eq!(jupiter_io::zk_snarks::altbn_128_pairing(2, 2), (0, 2, 0, 2));
    // /// ```
    fn altbn_128_pairing() -> bool {
        true
    }
}
