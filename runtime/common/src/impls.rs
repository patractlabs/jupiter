use frame_support::{
    parameter_types,
    traits::{Currency, Imbalance, OnUnbalanced},
};
use pallet_balances::NegativeImbalance;
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
use sp_runtime::{FixedPointNumber, Perquintill};

use patract_primitives::AccountId;

pub struct ToAuthor<R>(sp_std::marker::PhantomData<R>);

impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
where
    R: pallet_balances::Config + pallet_authorship::Config,
    <R as frame_system::Config>::AccountId: From<AccountId>,
    <R as frame_system::Config>::AccountId: Into<AccountId>,
    <R as frame_system::Config>::Event: From<pallet_balances::Event<R>>,
{
    fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
        let numeric_amount = amount.peek();
        let author = <pallet_authorship::Module<R>>::author();
        <pallet_balances::Module<R>>::resolve_creating(
            &<pallet_authorship::Module<R>>::author(),
            amount,
        );
        <frame_system::Module<R>>::deposit_event(pallet_balances::Event::Deposit(
            author,
            numeric_amount,
        ));
    }
}

parameter_types! {
    /// The portion of the `AvailableBlockRatio` that we adjust the fees with. Blocks filled less
    /// than this will decrease the weight and more will increase.
    pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
    /// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
    /// change the fees more rapidly.
    pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
    /// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
    /// that combined with `AdjustmentVariable`, we can recover from the minimum.
    /// See `multiplier_can_grow_from_zero`.
    pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}
/// Parameterized slow adjusting fee updated based on
/// https://w3f-research.readthedocs.io/en/latest/polkadot/Token%20Economics.html#-2.-slow-adjusting-mechanism
pub type SlowAdjustingFeeUpdate<R> =
    TargetedFeeAdjustment<R, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
