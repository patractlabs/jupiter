#![cfg_attr(not(feature = "std"), no_std)]

use contract::Module as Contracts;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use sp_core::crypto::UncheckedFrom;
use sp_std::vec::Vec;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config + contract::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    // type AccountId: contract::Config::AccountId;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Config> as TemplateModule
    where
        T::AccountId: AsRef<[u8]>,
        T::AccountId: UncheckedFrom<T::Hash>,
    {
        // Learn more about declaring storage items:
        // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        Something get(fn something): Option<u32>;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, AccountId),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Config>
    where
        T::AccountId: AsRef<[u8]>,
        T::AccountId: UncheckedFrom<T::Hash>,
    {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Config> for enum Call
    where
        origin: T::Origin,
        T::AccountId: AsRef<[u8]>,
        T::AccountId: UncheckedFrom<T::Hash>,
    {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn put_code(origin, code: Vec<u8>) -> dispatch::DispatchResult {
            <Contracts<T>>::put_code(origin, code)
        }

        // bls12_377

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_add(origin) {
            curve::tests::add(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_mul(origin) {
            curve::tests::mul(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_pairing_two(origin) {
            curve::tests::pairing(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_pairing_six(origin) {
            curve::tests::pairing_six(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_verify(origin) {
            curve::tests::verify(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_add(origin) {
            jupiter_io::pairing::bls12_377_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_mul(origin) {
            jupiter_io::pairing::bls12_377_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_pairing_two(origin) {
            jupiter_io::pairing::bls12_377_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_pairing_six(origin) {
            jupiter_io::pairing::bls12_377_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_verify(origin) {
            jupiter_io::pairing::bls12_377_verify();
        }

        // bls12_381

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_add(origin) {
            curve::tests::add(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_mul(origin) {
            curve::tests::mul(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_pairing_two(origin) {
            curve::tests::pairing(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_pairing_six(origin) {
            curve::tests::pairing_six(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_verify(origin) {
            curve::tests::verify(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_add(origin) {
            jupiter_io::pairing::bls12_381_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_mul(origin) {
            jupiter_io::pairing::bls12_381_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_pairing_two(origin) {
            jupiter_io::pairing::bls12_381_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_pairing_six(origin) {
            jupiter_io::pairing::bls12_381_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_verify(origin) {
            jupiter_io::pairing::bls12_381_verify();
        }

        // bn254

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_add(origin) {
            curve::tests::add(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_mul(origin) {
            curve::tests::mul(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_pairing_two(origin) {
            curve::tests::pairing(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_pairing_six(origin) {
            curve::tests::pairing_six(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_verify(origin) {
            curve::tests::verify(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_add(origin) {
            jupiter_io::pairing::bn254_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_mul(origin) {
            jupiter_io::pairing::bn254_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_pairing_two(origin) {
            jupiter_io::pairing::bn254_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_pairing_six(origin) {
            jupiter_io::pairing::bn254_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_verify(origin) {
            jupiter_io::pairing::bn254_verify();
        }

        // bw6_781

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_add(origin) {
            curve::tests::add(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_mul(origin) {
            curve::tests::mul(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_pairing_two(origin) {
            curve::tests::pairing(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_pairing_six(origin) {
            curve::tests::pairing_six(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_verify(origin) {
            curve::tests::verify(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_add(origin) {
            jupiter_io::pairing::bw6_761_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_mul(origin) {
            jupiter_io::pairing::bw6_761_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_pairing_two(origin) {
            jupiter_io::pairing::bw6_761_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_pairing_six(origin) {
            jupiter_io::pairing::bw6_761_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_verify(origin) {
            jupiter_io::pairing::bw6_761_verify();
        }
    }
}
