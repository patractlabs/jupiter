// This file is part of jupiter.

// Copyright (C) 2020-2021 Patract Labs Ltd.
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

//! # Randomness Module
//!
//! Store the babe randomness of each epoch and provide the babe randomness query by epoch id.
//!
//! ## Overview
//!
//! The Randomness module provide the historical babe randomness query by epoch id,
//! get the latest babe randomness, and the user input an additional value, hashing it with the
//! babe randomness to improve security.
//!
//! To use it in your runtime, you need to binding with pallet_session::SessionManager to trigger
//! the storage behavior of babe randomness when the session is changed.
//!
//! ### Goals
//!
//! The randomness module is designed to Provide a source of randomness for the ink! contract,
//! and expand this function through the chain extension mode of pallet contract.
//! We think that babe randomness are more secure than randomness-collective-flip.
//!
//!
//! ## Interface
//!
//! * `current_epoch`: Get babe randomness info for current epoch.
//! * `next_epoch`: Get babe randomness info for next epoch.
//! * `randomness_of`: Get babe randomness for historical epoch that before current epoch.
//! * `random`: Get randomness hashing with provider subject.
//!
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Babe`](../pallet_babe/index.html)

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::{decl_module, decl_storage, traits::Randomness as RandomnessT, Parameter};

use pallet_session::SessionManager;

use sp_consensus_vrf::schnorrkel;
use sp_runtime::traits::Member;
use sp_staking::SessionIndex;
use sp_std::prelude::*;

#[derive(Debug, PartialEq, Encode)]
pub struct BabeRandomness {
    pub epoch: u64,
    pub start_slot: u64,
    pub duration: u64,
    pub randomness: schnorrkel::Randomness,
}

pub trait Config: pallet_babe::Config + frame_system::Config {
    /// A stable ID for a validator.
    type ValidatorId: Member + Parameter;
}

decl_storage! {
    trait Store for Module<T: Config> as RandomnessExt {
        /// Historical randomness for each epoch.
        HistoricalRandomness get(fn historical_randomness): map hasher(twox_64_concat) u64 => schnorrkel::Randomness;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {}
}

/// An `SessionManager` implementation that wraps an inner `I` and also
/// sets the historical randomness of the ending session.
pub struct NoteHistoricalRandomness<T, I>(sp_std::marker::PhantomData<(T, I)>);

impl<T: Config, I> SessionManager<T::ValidatorId> for NoteHistoricalRandomness<T, I>
where
    I: SessionManager<T::ValidatorId>,
{
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::ValidatorId>> {
        <I as SessionManager<_>>::new_session(new_index)
    }

    fn end_session(end_index: SessionIndex) {
        <I as SessionManager<_>>::end_session(end_index);
        let epoch = <pallet_babe::Module<T>>::current_epoch();
        <HistoricalRandomness>::insert(epoch.epoch_index, epoch.randomness);
    }

    fn start_session(start_index: SessionIndex) {
        <I as SessionManager<_>>::start_session(start_index)
    }
}

impl<T: Config> Module<T> {
    // Return babe randomness info for current epoch
    pub fn current_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::current_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: epoch.randomness,
        }
    }

    // Return babe randomness info for next epoch
    pub fn next_epoch() -> BabeRandomness {
        let epoch = <pallet_babe::Module<T>>::next_epoch();
        BabeRandomness {
            epoch: epoch.epoch_index,
            start_slot: *epoch.start_slot,
            duration: epoch.duration,
            randomness: epoch.randomness,
        }
    }

    /// Return babe randomness for historical epoch
    pub fn randomness_of(epoch: u64) -> schnorrkel::Randomness {
        <HistoricalRandomness>::get(&epoch)
    }

    /// Return randomness with provider subject
    pub fn random(subject: &[u8]) -> T::Hash {
        <pallet_babe::Module<T>>::random(subject)
    }
}
