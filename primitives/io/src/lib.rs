//! Jupiter primitives - IO Module
#![deny(missing_docs)]
use sp_runtime_interface::runtime_interface;

/// ZK-SNARKs runtime interface
#[runtime_interface]
pub trait ZkSnarks {
    /// altbn_128 add
    fn altbn_128_add(lhs: u32, rhs: u32) -> u32 {
        lhs + rhs
    }

    /// altbn_128 mul
    fn altbn_128_mul(lhs: u32, rhs: u32) -> u32 {
        lhs + rhs
    }

    /// altbn_128 pairing
    fn altbn_128_pairing(lhs: u32, rhs: u32) -> (u32, u32, u32, u32) {
        (0, lhs, 0, rhs)
    }
}
