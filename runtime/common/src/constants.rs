//! A set of constant values used in substrate runtime.

// pub use time::*;

/// Money matters.
pub mod jupiter_currency {
    use super::jupiter_fee::{Weight2FeeNumerator, WeightToFee};
    use jupiter_primitives::Balance;

    pub const DOTS: Balance = 1_000_000_000_000; // old dot, one Dot is 100 Dot(new) now
    pub const DOLLARS: Balance = DOTS / 100; // 10_000_000_000 // one Dollar is 1 Dot(new) now
    pub const CENTS: Balance = DOLLARS / 100; // 100_000_000 // one Cent is 0.01 Dot(new) now
    pub const MILLICENTS: Balance = CENTS / 1_000; // 100_000 // one Millicent is 0.00001 Dot(new) now

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 20 * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
    }

    pub const fn tombstone_deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 10 * CENTS + (bytes as Balance) * 10 * MILLICENTS
    }

    pub struct JupiterNumerator;
    impl Weight2FeeNumerator for JupiterNumerator {
        const NUMERATOR: u128 = CENTS;
    }
    pub type JupiterWeight2Fee = WeightToFee<JupiterNumerator>;
}

pub mod currency {
    use jupiter_primitives::Balance;

    /// The existential deposit. Set to 1/10 of its parent Relay Chain.
    pub const EXISTENTIAL_DEPOSIT: Balance = 100 * MILLICENTS;

    pub const UNITS: Balance = 1_000_000_000_000;
    pub const CENTS: Balance = UNITS / 100;
    pub const MILLICENTS: Balance = CENTS / 1_000;
    pub const GRAND: Balance = CENTS * 100_000;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        // 1/10 of Westend testnet
        (items as Balance * 100 * CENTS + (bytes as Balance) * 5 * MILLICENTS) / 10
    }
}

/// Time.
pub mod time {
    use jupiter_primitives::{BlockNumber, Moment};

    pub const MILLISECS_PER_BLOCK: Moment = 6000;
    pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 1 * HOURS;

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;

    // 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}

// Common constants
// pub mod constants {
//     use frame_support::weights::{constants::WEIGHT_PER_SECOND, Weight};
//     use sp_runtime::Perbill;
// 
//     /// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
//     /// used to limit the maximal weight of a single extrinsic.
//     pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);
//     /// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
//     /// Operational  extrinsics.
//     pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
//     //
//     /// We allow for 0.5 seconds of compute with a 6 second average block time.
//     pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;
// }

/// Fee-related.
pub mod jupiter_fee {
    use frame_support::weights::{
        constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
        WeightToFeePolynomial,
    };
    use jupiter_primitives::Balance;
    use smallvec::smallvec;
    pub use sp_runtime::Perbill;

    /// The block saturation level. Fees will be updates based on this value.
    pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

    /// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
    /// node's balance type.
    ///
    /// This should typically create a mapping between the following ranges:
    ///   - [0, frame_system::MaximumBlockWeight]
    ///   - [Balance::min, Balance::max]
    ///
    /// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
    ///   - Setting it to `0` will essentially disable the weight fee.
    ///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
    pub struct WeightToFee<A: Weight2FeeNumerator>(sp_std::marker::PhantomData<A>);
    pub trait Weight2FeeNumerator {
        const NUMERATOR: u128;
    }
    impl<A: Weight2FeeNumerator> WeightToFeePolynomial for WeightToFee<A> {
        type Balance = Balance;
        fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
            // in Polkadot, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
            // let p = super::jupiter_currency::CENTS;
            let p = A::NUMERATOR;
            let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
            smallvec![WeightToFeeCoefficient {
                degree: 1,
                negative: false,
                coeff_frac: Perbill::from_rational(p % q, q),
                coeff_integer: p / q,
            }]
        }
    }
}

pub mod fee {
    use frame_support::weights::{
        constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
        WeightToFeePolynomial,
    };
    use jupiter_primitives::Balance;
    use smallvec::smallvec;
    pub use sp_runtime::Perbill;

    pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

    pub struct WeightToFee;
    impl WeightToFeePolynomial for WeightToFee {
        type Balance = Balance;
        fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
            let p = super::currency::CENTS;
            let q = 100 * Balance::from(ExtrinsicBaseWeight::get());
            smallvec![WeightToFeeCoefficient {
                degree: 1,
                negative: false,
                coeff_frac: Perbill::from_rational(p % q, q),
                coeff_integer: p / q,
            }]
        }
    }
}
