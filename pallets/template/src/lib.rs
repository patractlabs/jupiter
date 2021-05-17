// This file is part of Jupiter.

// Copyright (C) 2021-2021 Patract Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This is a template and benchmarks pallet with zkMega integration which contains:
//!
//!  * `add`
//!  * `mul`
//!  * `pairing`
//!  * `verify`
//!
//! methods for curves:
//!
//!  * bls12_377
//!  * bls12_381
//!  * bn254
//!  * bw6_781
//!
//! with both `native` and `wasm` executing environments.
//!
//! In this crate, the benchmark is run the benchmark code in benchmarking.rs file to get result.
//! And the call function is `decl_module` is only used for benchmarks.
//! Same to Ethereum zkp, we provide `add`, `mul` and `paring` 3 type for zkp benchmarks. Besides,
//! the `verify` represents the a wide range of scenarios benchmarks.
//!
//! For more details about zkMega, please check:
//! https://github.com/patractlabs/zkmega

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

use sp_core::crypto::UncheckedFrom;
use sp_runtime::traits::StaticLookup;
use sp_std::vec::Vec;

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, traits::Get, weights::Weight,
};
use pallet_contracts::{BalanceOf, CodeHash, Module as Contracts};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

decl_module! {
    pub struct Module<T: Config> for enum Call
    where
        origin: T::Origin,
        T::AccountId: AsRef<[u8]>,
        T::AccountId: UncheckedFrom<T::Hash>,
    {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// Proxy the call method of `pallet_contracts` for tests
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn call(
            origin,
            dest: <T::Lookup as StaticLookup>::Source,
            value: BalanceOf<T>,
            gas: Weight,
            data: Vec<u8>,
        ) -> dispatch::DispatchResultWithPostInfo {
            <Contracts<T>>::call(
                origin,
                dest,  // pallet_contracts address
                value, // T::Currency::minimum_balance() * 100u32.into(), // value
                gas,   // gas_limit
                data   // fn
            )
        }

        // bls12_377

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_add(_origin) {
            curve::tests::add(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_mul(_origin) {
            curve::tests::mul(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_pairing_two(_origin) {
            curve::tests::pairing(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_pairing_six(_origin) {
            curve::tests::pairing_six(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_377_verify(_origin) {
            curve::tests::verify(0x2a);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_add(_origin) {
            jupiter_io::pairing::bls12_377_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_mul(_origin) {
            jupiter_io::pairing::bls12_377_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_pairing_two(_origin) {
            jupiter_io::pairing::bls12_377_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_pairing_six(_origin) {
            jupiter_io::pairing::bls12_377_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_377_verify(_origin) {
            jupiter_io::pairing::bls12_377_verify();
        }

        // bls12_381

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_add(_origin) {
            curve::tests::add(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_mul(_origin) {
            curve::tests::mul(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_pairing_two(_origin) {
            curve::tests::pairing(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_pairing_six(_origin) {
            curve::tests::pairing_six(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bls12_381_verify(_origin) {
            curve::tests::verify(0x2b);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_add(_origin) {
            jupiter_io::pairing::bls12_381_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_mul(_origin) {
            jupiter_io::pairing::bls12_381_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_pairing_two(_origin) {
            jupiter_io::pairing::bls12_381_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_pairing_six(_origin) {
            jupiter_io::pairing::bls12_381_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bls12_381_verify(_origin) {
            jupiter_io::pairing::bls12_381_verify();
        }

        // bn254

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_add(_origin) {
            curve::tests::add(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_mul(_origin) {
            curve::tests::mul(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_pairing_two(_origin) {
            curve::tests::pairing(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_pairing_six(_origin) {
            curve::tests::pairing_six(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bn254_verify(_origin) {
            curve::tests::verify(0x2c);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_add(_origin) {
            jupiter_io::pairing::bn254_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_mul(_origin) {
            jupiter_io::pairing::bn254_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_pairing_two(_origin) {
            jupiter_io::pairing::bn254_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_pairing_six(_origin) {
            jupiter_io::pairing::bn254_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bn254_verify(_origin) {
            jupiter_io::pairing::bn254_verify();
        }

        // bw6_781

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_add(_origin) {
            curve::tests::add(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_mul(_origin) {
            curve::tests::mul(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_pairing_two(_origin) {
            curve::tests::pairing(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_pairing_six(_origin) {
            curve::tests::pairing_six(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn wasm_bw6_761_verify(_origin) {
            curve::tests::verify(0x2d);
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_add(_origin) {
            jupiter_io::pairing::bw6_761_add();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_mul(_origin) {
            jupiter_io::pairing::bw6_761_mul();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_pairing_two(_origin) {
            jupiter_io::pairing::bw6_761_pairing_two();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_pairing_six(_origin) {
            jupiter_io::pairing::bw6_761_pairing_six();
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn native_bw6_761_verify(_origin) {
            jupiter_io::pairing::bw6_761_verify();
        }
    }
}

//
// Custom stuffs for a pallet below
//

pub trait Config: frame_system::Config + pallet_contracts::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as TemplateModule
    where
        T::AccountId: AsRef<[u8]>,
        T::AccountId: UncheckedFrom<T::Hash>,
    {
        pub Contract: CodeHash<T>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        SomethingStored(u32, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config>
    where
        T::AccountId: AsRef<[u8]>,
        T::AccountId: UncheckedFrom<T::Hash>,
    {
        NoneValue,
        ContractDisappeared,
        InstantiateContractFailed
    }
}
