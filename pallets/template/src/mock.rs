use sp_runtime::{
    testing::{Header, H256},
    traits::{BlakeTwo256, Convert, Hash, IdentityLookup},
    AccountId32, Perbill,
};

use frame_support::{parameter_types, traits::Currency, weights::Weight};
use pallet_contracts::Frame;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use crate as pallet_template;
use crate::BalanceOf;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>},
        Template: pallet_template::{Pallet, Call, Storage, Event<T>},
    }
);
// Configure a mock runtime to test the pallet.
impl Convert<Weight, BalanceOf<Self>> for Test {
    fn convert(w: Weight) -> BalanceOf<Self> {
        w as u128
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId32;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
    pub static ExistentialDeposit: u128 = 0;
    pub const MaxReserves: u32 = 50;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u128;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const SignedClaimHandicap: u64 = 2;
    pub const TombstoneDeposit: u64 = 16;
    pub const DepositPerContract: u64 = 8 * DepositPerStorageByte::get();
    pub const DepositPerStorageByte: u64 = 10_000;
    pub const DepositPerStorageItem: u64 = 10_000;
    pub RentFraction: Perbill = Perbill::from_rational(4u32, 10_000u32);
    pub const SurchargeReward: u64 = 150;
    pub const MaxDepth: u32 = 100;
    pub const DeletionQueueDepth: u32 = 1024;
    pub const DeletionWeightLimit: Weight = 500_000_000_000;
    pub Schedule: pallet_contracts::Schedule<Test> = Default::default();
}

impl pallet_contracts::Config for Test {
    type Time = Timestamp;
    type Randomness = Randomness;
    type Currency = Balances;
    type Event = Event;
    type RentPayment = ();
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type DepositPerContract = DepositPerContract;
    type DepositPerStorageByte = DepositPerStorageByte;
    type DepositPerStorageItem = DepositPerStorageItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type CallStack = [Frame<Self>; 31];
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = jupiter_chain_extension::JupiterExt;
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = Schedule;
}

impl pallet_template::Config for Test {
    type Event = Event;
}

impl pallet_randomness_collective_flip::Config for Test {}

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

pub struct ExtBuilder {
    existential_deposit: u128,
}
impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: 1,
        }
    }
}

impl ExtBuilder {
    pub fn set_associated_consts(&self) {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
    }

    pub fn existential_deposit(mut self, existential_deposit: u128) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut t)
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

/// Compile groth16 with test data
pub fn groth16_addr(caller: AccountId32) -> AccountId32 {
    let module = include_bytes!("../res/groth16.wasm").to_vec();
    let hash = <Test as frame_system::Config>::Hashing::hash(&module);
    let _ = crate::mock::Balances::deposit_creating(&caller, 100_000_000_000_000);
    Contracts::instantiate_with_code(
        Origin::signed(ALICE.clone()),
        1_000_000,                   // endowment
        Weight::max_value(),         // gas_limit
        module,                      // code
        [106, 55, 18, 226].to_vec(), // flip
        b"".to_vec(),                // salt
    )
    .unwrap();
    let addr = Contracts::contract_address(&caller, &hash, &b"".to_vec());
    addr
}
