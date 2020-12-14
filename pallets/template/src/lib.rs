#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Config> as TemplateModule {
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
    pub enum Error for Module<T: Config> {
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
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;


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


        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;

            // Update storage.
            Something::put(something);

            // Emit an event.
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            // Return a successful DispatchResult
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn cause_error(origin) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match Something::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    Something::put(new);
                    Ok(())
                },
            }
        }
    }
}