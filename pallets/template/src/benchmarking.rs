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

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use pallet_contracts::{Module as Contracts, *};
use sp_std::prelude::*;

benchmarks! {
    _ { }
    // ink_groth16_verify {
    //     // let caller: T::AccountId = whitelisted_caller();
    //     let salt = vec![0xff];
    //     Contracts<T>::put_code_raw(&[]);
    //     Contracts::<T>::instantiate(
    //         RawOrigin::Signed(caller.clone()).into(),
    //         endowment,
    //         Weight::max_value(),
    //         module.hash,
    //         data,
    //         salt,
    //     )?;
    // } // : bare_call(caller, )
    // call {
    //     let data = vec![42u8; 1024];
    //     let instance = Contract::<T>::with_caller(
    //         whitelisted_caller(), WasmModule::dummy_with_mem(), vec![], Endow::CollectRent
    //     )?;
    //     let value = T::Currency::minimum_balance() * 100u32.into();
    //     let origin = RawOrigin::Signed(instance.caller.clone());
    //     let callee = instance.addr.clone();
    //
    //     // trigger rent collection for worst case performance of call
    //     System::<T>::set_block_number(instance.eviction_at()? - 5u32.into());
    //     let before = T::Currency::free_balance(&instance.account_id);
    // }: _(origin, callee, value, Weight::max_value(), data)
    // verify {
    //     // endowment and value transfered via call should be removed from the caller
    //     assert_eq!(
    //         T::Currency::free_balance(&instance.caller),
    //         caller_funding::<T>() - instance.endowment - value,
    //     );
    //     // rent should have lowered the amount of balance of the contract
    //     assert!(T::Currency::free_balance(&instance.account_id) < before + value);
    //     // but it should not have been evicted by the rent collection
    //     instance.alive_info()?;
    // }

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
