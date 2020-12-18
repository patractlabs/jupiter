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
        let module = include_bytes!(concat!(env!("OUT_DIR"), "/flip.wasm")).to_vec();
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
            endowment, // endowment
            Weight::max_value(), // gas_limit
            hash.clone(), // code_hash
            [106, 55, 18, 226].to_vec(), // flip
            b"".to_vec(), // salt
        ).map_err(|_|<Error<T>>::InstantiateContractFailed)?;

        // Calculate addr
        let addr = Contracts::<T>::contract_address(&caller, &hash, &b"".to_vec());
    }: call(origin, addr, hex::decode("4e1f2d81088501e60f6032ed7f170ba3481c5d99c0890554c808ad380a7c9aefc9e62eeecf0363ecef5e59ba8eaf6dd0af40e08d7cfd024314f90080a1461a207aa3098308149837bc6322a8f4b499671c507203e798f90a9aa8fafaebb806c8708c3207a93316008501ea7f398187aecf2721441218e54645b17acd35bdf65886255e772a58353ef2e25c92cf7f872bea5afca6081d03e6b115e74befd85720c241ee5ef038621a1cc72e329db9aebec0faf480e83cbd9e4b735650ab0cce2e321d64311bd34bca9e0200910a9094c864d37f890720969430f2682ccc76dc83f9234694590bd334b40dbfa7f28de40e914571d8ae2a88712844f86f0d6dd2f4875120f66304bfcedd55c9a805310155a5345ddc626cbb6fe5ce7026be174a08c798e4d1f1010f97138c30b10a0e23d3cde62852c7b2aabcd7603f87fc54cb0c2fcfa48a1a6e07ea917eec248284a1409dd21ae7bd64b862e7df91fd0af1f36857260a843ea18cd393172b91196ca17bc5f37efb126d585abfcf23b8438d72f90365c382b661256d7caa361d07000c06c5cc02155140b97283cc024daf57023a781a9115bb6e17350264f8b52784a6008921402af8c05dd39f4b789bd50435d2c5aefeb6dc9817af41f45779e3a2d80ab98ef923723b938566747c10a4a169ca5097c5d5073449c3d2e65e16291850a5721416e102df69729ffab67cf948ca51c4ac4c2b54c521552fd4a5a5462d6d3b77ba9932aba0fda8ea8da76e5b0b9166ba600f9c5cc5e442b64ed560c126f8b88e2761c0c6a17a4dcbc1a132a975683ed0b6018ac45d7f4942ad7831790e00408697568bc15871da494a2fe12199531130a8a5b96f07c69020dc01bf9f3ed043a6e1bd08f8d7826c7d5c0fc5c771195ab91bf6eeaa33cbd89cd7dce7930b383c2453d0cca5c227e1b97b1ae253e40cadf4468d6d86f0bf69a0b0bb55774219005b6d86316ea7c1dc4df3e3396a9f78aadf416d5af1619883f5dbbf0a39c6743b194e5adedda54d7cb3ae33e556602e19dd43a31bca7bdb15ad5245dc1ad0affb97edc8f35bb00d1c5b417a132d5be9b72a39870193b6a142198a888c0fe3760fdf1467c7208394270f2aff044c5c002d6cf189a28e34851dc434933e73e07482dad6c7ab931bfbdf29f5d51a6a866d028b91cd89abc9728cf2f8e139f612b5a248ca0758365d1d83c6bc299152b15a89b136e804ae71174d4f35970be570fd14000d06fcafa9753aee681f006d7bf6fc885dcc1a70c33bbfe92bb581ac51e1325b782b1128b1586767cb2ea27555e243abd213634b87418421a65002e06fd9347b23205761d7e35a2a83ffcd35b16b6bf23982700597f2ed9706ae5e039542e8eabd1800443cc8d6ffec17c44207dd41e8ab2f2f2c663f5ed9ab724321de43cbe2ea81908c13643c42a41be1adbc5076e4927b13046d3cdaf5ef775ee48d842599508178276cd41d30e289cbd1d2bb9f92f59b520138fef3e9529139f3961f40f6ea92113ed1701b0eb62a8b21b0852f5ae0d23b4d8cf7a02a69b04abbf0b9213cc1cd16b0afed74a09f1d965f72e0faaad60215c2788b9672a8f53fb90d44b47fb16e4f97f910226d8ad46ddc1dfa6eb4f709a3c95ccfa7f4cd1b9ea5f3cf3d10c5dd0200de094abc5e25fac259efcfd79a97401359069cde3b48eaa9d3a4d848959eb269d788a415d9c22fb418459474f1f0fc16f4572ace2876cf1170d400d1d12d588f816023602a077b8fa66293ac19c951b6c572fad4481fa576e362d9f3a0ffa0120004801d6c7850edbac8a5281ab93d2ed245d47b64f20c21950926d595624b488c291c").unwrap())

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
