// Copyright (C) 2021-2021 Patract Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

//! Test utilities

use crate as poa;
use crate::*;
use frame_support::{
    parameter_types,
    traits::{FindAuthor, Get, OnFinalize, OnInitialize, OneSessionHandler},
    weights::constants::RocksDbWeight,
};
use sp_core::H256;
use sp_io;
use sp_runtime::{
    testing::{Header, TestXt, UintAuthorityId},
    traits::{IdentityLookup, Zero},
    Perbill,
};
use sp_staking::offence::{OffenceDetails, OnOffenceHandler};
use sp_staking::SessionIndex;
use std::{cell::RefCell, collections::HashSet};

pub const INIT_TIMESTAMP: u64 = 30_000;
pub const BLOCK_TIME: u64 = 1000;
pub const ENDOWMENT: u128 = 1000;

/// The AccountId alias in this test module.
pub(crate) type AccountId = u64;
pub(crate) type AccountIndex = u64;
pub(crate) type BlockNumber = u64;
pub(crate) type Balance = u128;

thread_local! {
    static SESSION: RefCell<(Vec<AccountId>, HashSet<AccountId>)> = RefCell::new(Default::default());
}

/// Another session handler struct to test on_disabled.
pub struct OtherSessionHandler;
impl OneSessionHandler<AccountId> for OtherSessionHandler {
    type Key = UintAuthorityId;

    fn on_genesis_session<'a, I: 'a>(_: I)
    where
        I: Iterator<Item = (&'a AccountId, Self::Key)>,
        AccountId: 'a,
    {
    }

    fn on_new_session<'a, I: 'a>(_: bool, validators: I, _: I)
    where
        I: Iterator<Item = (&'a AccountId, Self::Key)>,
        AccountId: 'a,
    {
        SESSION.with(|x| {
            *x.borrow_mut() = (validators.map(|x| x.0.clone()).collect(), HashSet::new())
        });
    }

    fn on_disabled(validator_index: usize) {
        SESSION.with(|d| {
            let mut d = d.borrow_mut();
            let value = d.0[validator_index];
            d.1.insert(value);
        })
    }
}

impl sp_runtime::BoundToRuntimeAppPublic for OtherSessionHandler {
    type Public = UintAuthorityId;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        PoA: poa::{Module, Call, Config<T>, Storage, Event<T>},
        Session: pallet_session::{Module, Call, Storage, Event, Config<T>},
    }
);

/// Author of block is always 11
pub struct Author11;
impl FindAuthor<AccountId> for Author11 {
    fn find_author<'a, I>(_digests: I) -> Option<AccountId>
    where
        I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
    {
        Some(11)
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            frame_support::weights::constants::WEIGHT_PER_SECOND * 2
        );
    pub const MaxLocks: u32 = 1024;
    pub static ExistentialDeposit: Balance = 1;
    pub static Period: BlockNumber = 5;
    pub static Offset: BlockNumber = 0;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = RocksDbWeight;
    type Origin = Origin;
    type Index = AccountIndex;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}
impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}
parameter_types! {
    pub const UncleGenerations: u64 = 0;
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(25);
}
sp_runtime::impl_opaque_keys! {
    pub struct SessionKeys {
        pub other: OtherSessionHandler,
    }
}
impl pallet_session::Config for Test {
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Test, PoA>;
    type Keys = SessionKeys;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionHandler = (OtherSessionHandler,);
    type Event = Event;
    type ValidatorId = AccountId;
    type ValidatorIdOf = poa::SimpleValidatorIdConverter<AccountId>;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type WeightInfo = ();
}

impl pallet_session::historical::Config for Test {
    type FullIdentification = AccountId;
    type FullIdentificationOf = poa::SimpleValidatorIdConverter<AccountId>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub MinimumAuthorities: u32 = PoA::minimum_authority_count();
    pub const SessionsPerEra: SessionIndex = 1;
    pub const SlashDeferDuration: poa::EraIndex = 2 * SessionsPerEra::get();
}

parameter_types! {
    pub const CouncilMotionDuration: BlockNumber = 3;
    pub const CouncilMaxProposals: u32 = 100;
    pub const CouncilMaxMembers: u32 = 100;
}

impl poa::Config for Test {
    type MinimumAuthorities = MinimumAuthorities;
    type SessionsPerEra = SessionsPerEra;
    type SlashDeferDuration = SlashDeferDuration;
    type UnixTime = Timestamp;
    type CouncilOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type SlashCancelOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type SessionInterface = Self;
    type Event = Event;
    type WeightInfo = (); //TODO change this
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

pub type Extrinsic = TestXt<Call, ()>;

pub struct ExtBuilder {
    minimum_authority_count: u32,
    initial_authorities: Vec<AccountId>,
    initial_invulnerable: Vec<AccountId>,
    initialize_first_session: bool,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            minimum_authority_count: 2,
            initial_authorities: vec![1, 2],
            initial_invulnerable: vec![1, 2],
            initialize_first_session: true,
        }
    }
}

impl ExtBuilder {
    pub fn minimum_authority_count(mut self, count: u32) -> Self {
        self.minimum_authority_count = count;
        self
    }
    pub fn initial_authorities(mut self, initial_authorities: Vec<AccountId>) -> Self {
        self.initial_authorities = initial_authorities;
        self
    }
    pub fn build(self) -> sp_io::TestExternalities {
        sp_tracing::try_init_simple();
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let validators = self.initial_authorities.clone();

        let _ = pallet_balances::GenesisConfig::<Test> {
            balances: validators
                .iter()
                .map(|k: &AccountId| (k.clone(), ENDOWMENT))
                .collect(),
        }
        .assimilate_storage(&mut storage);

        let _ = poa::GenesisConfig::<Test> {
            minimum_authority_count: self.minimum_authority_count,
            init_authorities: self.initial_authorities.clone(),
            init_invulnerables: self.initial_invulnerable,
            ..Default::default()
        }
        .assimilate_storage(&mut storage);

        let _ = pallet_session::GenesisConfig::<Test> {
            keys: validators
                .iter()
                .map(|x| {
                    (
                        *x,
                        *x,
                        SessionKeys {
                            other: UintAuthorityId(*x as u64),
                        },
                    )
                })
                .collect(),
        }
        .assimilate_storage(&mut storage);

        let mut ext = sp_io::TestExternalities::from(storage);
        ext.execute_with(|| {
            let validators = Session::validators();
            SESSION.with(|x| *x.borrow_mut() = (validators.clone(), HashSet::new()));
        });

        if self.initialize_first_session {
            // We consider all test to start after timestamp is initialized This must be ensured by
            // having `timestamp::on_initialize` called before `staking::on_initialize`. Also, if
            // session length is 1, then it is already triggered.
            ext.execute_with(|| {
                System::set_block_number(1);
                Session::on_initialize(1);
                Timestamp::set_timestamp(INIT_TIMESTAMP);
            });
        }

        ext
    }
    pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
        let mut ext = self.build();
        ext.execute_with(test);
    }
}

pub(crate) fn active_era() -> EraIndex {
    PoA::active_era().unwrap().index
}

pub(crate) fn current_era() -> EraIndex {
    PoA::current_era().unwrap()
}

/// Progress to the given block, triggering session and era changes as we progress.
///
/// This will finalize the previous block, initialize up to the given block, essentially simulating
/// a block import/propose process where we first initialize the block, then execute some stuff (not
/// in the function), and then finalize the block.
pub(crate) fn run_to_block(n: BlockNumber) {
    <PoA as OnFinalize<u64>>::on_finalize(System::block_number());
    for b in (System::block_number() + 1)..=n {
        System::set_block_number(b);
        Session::on_initialize(b);
        Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
        if b != n {
            <PoA as OnFinalize<u64>>::on_finalize(System::block_number());
        }
    }
}

/// Progresses from the current block number (whatever that may be) to the `P * session_index + 1`.
pub(crate) fn start_session(session_index: SessionIndex) {
    let end: u64 = if Offset::get().is_zero() {
        (session_index as u64) * Period::get()
    } else {
        Offset::get() + (session_index.saturating_sub(1) as u64) * Period::get()
    };
    run_to_block(end);
    // session must have progressed properly.
    assert_eq!(
        Session::current_index(),
        session_index,
        "current session index = {}, expected = {}",
        Session::current_index(),
        session_index,
    );
}

/// Progress until the given era.
pub(crate) fn start_active_era(era_index: EraIndex) {
    start_session((era_index * <SessionsPerEra as Get<u32>>::get()).into());
    assert_eq!(active_era(), era_index);
    // One way or another, current_era must have changed before the active era, so they must match
    // at this point.
    assert!(current_era() >= active_era());
}

pub(crate) fn on_offence_in_era(
    offenders: &[OffenceDetails<
        AccountId,
        pallet_session::historical::IdentificationTuple<Test>,
    >],
    slash_fraction: &[Perbill],
    era: EraIndex,
) {
    if active_era() == era {
        let _ = PoA::on_offence(
            offenders,
            slash_fraction,
            PoA::eras_start_session_index(era).unwrap(),
        )
        .unwrap();
    } else {
        panic!("cannot slash in era {}", era);
    }
}

pub(crate) fn on_offence_now(
    offenders: &[OffenceDetails<
        AccountId,
        pallet_session::historical::IdentificationTuple<Test>,
    >],
    slash_fraction: &[Perbill],
) {
    let now = active_era();
    on_offence_in_era(offenders, slash_fraction, now)
}

pub(crate) fn add_slash(who: &AccountId) {
    on_offence_now(
        &[OffenceDetails {
            offender: (who.clone(), who.clone()),
            reporters: vec![],
        }],
        &[Perbill::default()],
    );
}

pub(crate) fn poa_events() -> Vec<poa::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let Event::poa(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect()
}
