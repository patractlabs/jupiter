//! Jupiter primitives - IO Module
#![deny(missing_docs)]
use sp_runtime_interface::runtime_interface;

/// ZK-SNARKs runtime interface
#[runtime_interface]
pub trait ZkSnarks {
    /// altbn_128 add
    ///
    /// ```
    /// assert_eq!(jupiter_io::zk_snarks::altbn_128_add(1, 1), 2);
    /// ```
    fn altbn_128_add(lhs: u32, rhs: u32) -> u32 {
        lhs + rhs
    }

    /// altbn_128 mul
    ///
    /// ```
    /// assert_eq!(jupiter_io::zk_snarks::altbn_128_mul(2, 2), 4);
    /// ```
    fn altbn_128_mul(lhs: u32, rhs: u32) -> u32 {
        lhs * rhs
    }

    /// altbn_128 pairing
    /// ```
    /// assert_eq!(jupiter_io::zk_snarks::altbn_128_pairing(2, 2), (0, 2, 0, 2));
    /// ```
    fn altbn_128_pairing(lhs: u32, rhs: u32) -> (u32, u32, u32, u32) {
        (0, lhs, 0, rhs)
    }
}
