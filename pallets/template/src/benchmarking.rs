// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
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

//! Balances pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use contract::{BalanceOf, ConfigCache, Module as Contracts};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::{traits::Currency, weights::Weight};
use frame_system::RawOrigin;
use sp_runtime::traits::{Bounded, CheckedDiv, Hash, Saturating, Zero};
use sp_std::prelude::*;

benchmarks! {
    where_clause { where
        T::AccountId: UncheckedFrom<T::Hash>,
        T::AccountId: AsRef<[u8]>,
    }
    _ {}

    test_contract {
        let caller: T::AccountId = whitelisted_caller();
        let origin = RawOrigin::Signed(caller.clone());
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        // Put WASM module
        let module = include_bytes!(concat!(env!("OUT_DIR"), "/groth16.wasm")).to_vec();
        let hash = T::Hashing::hash(&module);
        <Contracts<T>>::put_code(origin.clone().into(), module)?;

        // storage_size cannot be zero because otherwise a contract that is just above
        // the subsistence threshold does not pay rent given a large enough subsistence
        // threshold. But we need rent payments to occur in order to benchmark for worst cases.
        let storage_size = ConfigCache::<T>::subsistence_threshold_uncached()
            .checked_div(&T::RentDepositOffset::get())
            .unwrap_or_else(Zero::zero);

        // Endowment should be large but not as large to inhibit rent payments.
        let endowment = T::RentDepositOffset::get()
            .saturating_mul(storage_size + T::StorageSizeOffset::get().into())
            .saturating_sub(1u32.into());

        <Contracts<T>>::instantiate(
            origin.clone().into(),
            endowment.clone(), // endowment
            Weight::max_value(), // gas_limit
            hash.clone(), // code_hash
            [106, 55, 18, 226].to_vec(), // flip
            b"".to_vec(), // salt
        ).map_err(|_|<Error<T>>::InstantiateContractFailed)?;

        // Calculate addr
        let addr = Contracts::<T>::contract_address(&caller, &hash, &b"".to_vec());
    }: call(origin, addr, endowment, Weight::max_value(), vec![192, 150, 165, 243])

    wasm_bls12_377_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_377_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_377_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_377_pairing_two(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_377_pairing_six(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_377_verify(RawOrigin::Signed(caller.clone()))
        verify {}

    // bls12_381

    wasm_bls12_381_add {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bls12_381_add(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bls12_381_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bls12_381_pairing_two(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bls12_381_pairing_six(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bls12_381_verify(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_add {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_381_add(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_381_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_381_pairing_two(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_381_pairing_six(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bls12_381_verify(RawOrigin::Signed(caller.clone()))
        verify {}

    // bn254

    wasm_bn254_add {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bn254_add(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bn254_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bn254_pairing_two(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bn254_pairing_six(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bn254_verify(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_add {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bn254_add(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bn254_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bn254_pairing_two(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bn254_pairing_six(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bn254_verify(RawOrigin::Signed(caller.clone()))
        verify {}

    // bw6_761

    wasm_bw6_761_add {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bw6_761_add(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bw6_761_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bw6_761_pairing_two(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bw6_761_pairing_six(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bw6_761_verify(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_add {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bw6_761_add(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bw6_761_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bw6_761_pairing_two(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bw6_761_pairing_six(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: native_bw6_761_verify(RawOrigin::Signed(caller.clone()))
        verify {}
}
