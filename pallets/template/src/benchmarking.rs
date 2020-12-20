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
// #![recursion_limit = "256"]

use super::*;

use contract::{BalanceOf, Module as Contracts};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::{traits::Currency, weights::Weight};
use frame_system::RawOrigin;
use sp_runtime::traits::{Bounded, Hash};
use sp_std::prelude::*;

macro_rules! addr {
    () => {{
        let caller: T::AccountId = whitelisted_caller();
        let origin = RawOrigin::Signed(caller.clone());
        T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

        // Put WASM module
        let module = include_bytes!("../res/groth16.wasm").to_vec();
        let hash = T::Hashing::hash(&module);
        <Contracts<T>>::put_code(origin.clone().into(), module)?;
        if let Err(e) = <Contracts<T>>::instantiate(
            origin.clone().into(),
            BalanceOf::<T>::max_value() / 2_u32.into(), // endowment
            Weight::max_value(),                        // gas_limit
            hash.clone(),                               // code_hash
            [106, 55, 18, 226].to_vec(),                // deafult
            b"".to_vec(),                               // salt
        ) {
            panic!("{:?}", e);
        }

        // Calculate addr
        (
            Contracts::<T>::contract_address(&caller, &hash, &b"".to_vec()),
            caller,
        )
    }};
}

benchmarks! {
    where_clause { where
        T::AccountId: UncheckedFrom<T::Hash>,
        T::AccountId: AsRef<[u8]>,
    }
    _ {}

    bls12_377_groth16_verify {
        let (addr, caller) = addr!();
        let data = hex::decode("4e1f2d8108850171c5a779bb19e0aeb6030768f5001017bbd4edb645306ff1591424c501e7aa8b480fc18d8ed404c893d473afa9b82c01fb3e19bdf85f4fb8acf97d1507b75eb48972c8a10e13f4637f7bee71f89eafc141453e4646c82e188d9ef8cf71fa2f000085014d0869345487bb4070110fa318b0f408dcffce12304589878797615d8a72cc00b6a1e3f50cc0e59770638640090726013db560d3a19288ecedc60ed9ee0afd50ad3ac96db89d99fc249b85a453ef837e9213f8977ac572d818dc43387678a50100910a54f796d78550d14c4ee03cd5915fd8037569696f878312b0093f0149919fc717c0b3f8a2d419fad0edde7e1d55cd2f002c312b660250225a00baf5df511dcce61eb23b28f3980be7fcde0091284670e9064015f452f244e65cad047a20fdd000ca76892d04e1283e575e098c8e4281ba9113443515f22247921b9c3189dc03dd83cf22b9da3eda70cdd2644a795376002e0b1d0d1d5945dab12a28a31497b48d07739028ac3153be3cf367e3d7e788ac43f6d3ccd3c5c20a4f2d6b0db05ce40000f70cb4191a256ed9a4d90305f5864993032a0e40433040c259b14c8069d117f3491d75d993271a0a3f13efdd5b18ce006d90eff3b45f8fa10b43a2d4fb80a554304ba5d94ddc19c92b2e1e2f756bd3042c4c0d6e10b6c22c2ea5eb136fb42900aca080ed5cfa9ccf92da7e4f631f1f9fd52a4dd227898e21585a7a55656f6315fa3f1e541011b941084fb663515ede0024ff6c7745872af24c5de2acfbf4b7a238f3f80face2310d83d9e953de2051a70abc3be988220291e03c92840e1c1c01005720823639fe8f3818ade6c24c6d6ed8f78b87369a9fac8a548e01ca001e424ba931033958641fe06677c3f4a1b39f01f6d3e9a2052ad1185af9a80086665df50b41674f7f6f181773cee6e78d09ea34ce970ccb5907d5cb2f69c9e0ed8d1f01009d629094c47bdf026bbd3424f6e7748b0c3529b495cf24677f309d9f1ed7d3435513224cba204902fd733522b3b855015f395e5b2a7f0fe9e50665d705c48f9b8b20a3f9619f50c1dcddbea7c4a561cdffed750045f875b2d2c569e8d5d0320015e739c8881a211865f82e39bb6ad21a66ceb1808a5ecc3f11833d61dc938a3b160b1b1829527b52e9d3d98800f29701078dfc5fda2f3dcebaca0f43ac23f6da5044f51cf12ac8b3439df503ab1732f5047601b2cd9d93e6d161dc4719beda00000d06e2342d048b06618cfc80a5fb5301373032d5780ca45e5287e824fd1d3dccfc36d1befe73edaea7378968a003f94e1801de729f41f406c7d94db4995cc2956cdd6b250937dd52b3c8ac812dff25985eb802c727dbfb7c6b055db545d687b1bd000090f4ad7dc16060ebfbe2bff08077e404e878850682e15f64a5b0f243f53a6d61dc358d41fa6aed615c6296904d71c200c04fe75b3adbecdc3d00df22556a1c949e4b87c81483225f6eaa7560e3035a1c919c4a0d446125dca2e2ef541d58a200ec9b0644bea1bd34e84ff04b65c0cc8733d425b43225f81a7580e1027a41b4ce85eb513213f54a76e2b69320dc6d51003b06cb148aa149b0b0e1ffb8f5901adbda3e431c19404cae29f26aac97085c6d1d8eeef33024aafdc649da6475ab99000026b3bb740a34b4db3a1c689d471b5a7834bb27fd57108bca8cd88d12bb81eae5f8ad807e5de758978ab51d0146017100c31d5c7971576e79eec56286dc2602025cb1d0a01e1acdc23a8aece6e9f3c3ecf97d0a9667634e31e79f653a93c13b01000480a4961fec1c734055a80af91b3fd7405a2cdb741e1867aa7f783ca0959a077110").expect("Decode failed");
    }: call(
        RawOrigin::Signed(caller),
        T::Lookup::unlookup(addr),
        0_u32.into(),
        Weight::max_value(),
        data
    )

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
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_377_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    // bls12_381

    wasm_bls12_381_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bls12_381_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bls12_381_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    // bn254

    wasm_bn254_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: wasm_bn254_mul(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bn254_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bn254_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    // bw6_761

    wasm_bw6_761_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    wasm_bw6_761_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_add {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_mul {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_pairing_two {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_pairing_six {
        let caller: T::AccountId = whitelisted_caller();
    }:  _(RawOrigin::Signed(caller.clone()))
        verify {}

    native_bw6_761_verify {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()))
        verify {}
}
