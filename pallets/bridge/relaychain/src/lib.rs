use sp_runtime::{
    traits::{
        AtLeast32BitUnsigned, Bounded, CheckedAdd, CheckedSub, MaybeSerializeDeserialize,
        Saturating, StaticLookup, StoredMapError, Zero,
    },
    DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::prelude::*;

pub use weights::WeightInfo;

mod chain;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        // /// The overarching event type.
        // type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    // #[pallet::event]
    // #[pallet::generate_deposit(pub(super) fn deposit_event)]
    // #[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
    // pub enum Event<T: Config<I>, I: 'static = ()> {
    // }

    #[pallet::error]
    pub enum Error<T, I = ()> {
        Decode
    }

    #[pallet::storage]
    #[pallet::getter(fn relayers)]
    pub type Relayers<T: Config<I>, I: 'static = ()> =
        StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        #[pallet::weight(T::WeightInfo::submit_ksm_extrinsic())]
        pub fn submit_ksm_extrinsic(
            origin: OriginFor<T>,
            raw_extrinsic: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            use crate::chain::kusama::KsmExtrinsic;
            use codec::Decode;
            let e = KsmExtrinsic::decode(&mut &raw_extrinsic[..]).map_err(|_| Error::<T, I>::Decode)?;
            Ok(().into())
        }
    }
}
