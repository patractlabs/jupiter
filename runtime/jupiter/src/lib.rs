#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod chain_extension;

use sp_api::impl_runtime_apis;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, Convert},
    transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
};
use sp_std::marker::PhantomData;

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
    construct_runtime, match_type, parameter_types,
    traits::{All, Contains, Get},
    weights::{constants::RocksDbWeight, IdentityFee, Weight},
    PalletId,
};
use frame_system::EnsureRoot;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use xcm::v0::{Junction, MultiAsset, MultiLocation, NetworkId, Xcm};
use xcm_builder::{
    AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, CurrencyAdapter,
    EnsureXcmOrigin, FixedWeightBounds, IsConcrete, LocationInverter, NativeAsset,
    ParentAsSuperuser, ParentIsDefault, RelayChainAsNative, SiblingParachainAsNative,
    SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
    SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
};
use xcm_executor::{
    traits::{FilterAssetLocation, ShouldExecute},
    Config, XcmExecutor,
};

pub use jupiter_primitives::{
    AccountId, Amount, Balance, BlockNumber, CurrencyId, Hash, Header, Index, ReserveIdentifier,
    Signature,
};
use jupiter_runtime_common::*;
use jupiter_runtime_common::{
    constants::{jupiter_currency::*, time::*},
    impls, weights,
};

use pallet_contracts::weights::WeightInfo;
use pallet_contracts_primitives::ContractExecResult;

use crate::chain_extension::JupiterParaExtension;
use randomness_collect::sr25519::AuthorityId as RandomCollectId;
pub use randomness_collect::{RpcPort, OCW_DB_RANDOM};

pub use cumulus_primitives_core::ParaId;

use orml_xcm_support::MultiNativeAsset;

pub use sp_runtime::{Perbill, Percent, Permill, Perquintill};

pub mod opaque {
    use super::*;
    pub type SessionHandlers = ();

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
        }
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("jupiter-westend"),
    impl_name: create_runtime_str!("patract-jupiter-westend"),
    authoring_version: 1,
    spec_version: 2,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type AccountId = AccountId;
    type Call = Call;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Event = Event;
    type Origin = Origin;
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
}

// ParaChain slot duration
parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = (CollatorSelection,);
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
    type MaxReserves = MaxReserves;
    type MaxLocks = MaxLocks;
    // type ReserveIdentifier = [u8; 8];
    type ReserveIdentifier = ReserveIdentifier;
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = JupiterWeight2Fee;
    type FeeMultiplierUpdate = impls::SlowAdjustingFeeUpdate<Self>;
}

parameter_types! {
    pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
        )) / 5) as u32;
    pub Schedule: pallet_contracts::Schedule<Runtime> = Default::default();
}

impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
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
    type CallStack = [pallet_contracts::Frame<Self>; 31];
    type WeightPrice = pallet_transaction_payment::Pallet<Self>;
    // type WeightPrice = module_transaction_payment::Pallet<Self>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    type ChainExtension = JupiterParaExtension<Self>;
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type Schedule = Schedule;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}

impl pallet_utility::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type Event = Event;
    type OnValidationData = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type DmpMessageHandler = DmpQueue;
    type ReservedDmpWeight = ReservedDmpWeight;
    type OutboundXcmpMessageSource = XcmpQueue;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
    pub const WestendLocation: MultiLocation = MultiLocation::X1(Junction::Parent);
    pub RelayNetwork: NetworkId = NetworkId::Named(b"Westend".to_vec());
    pub RelayChainOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
    pub Ancestry: MultiLocation = Junction::Parachain(ParachainInfo::parachain_id().into()).into();
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
    // The parent (Relay-chain) origin converts to the default `AccountId`.
    ParentIsDefault<AccountId>,
    // Sibling parachain origins convert to AccountId via the `ParaId::into`.
    SiblingParachainConvertsVia<Sibling, AccountId>,
    // Straight up local `AccountId32` origins just alias directly to `AccountId`.
    AccountId32Aliases<RelayNetwork, AccountId>,
);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
    // Sovereign account converter; this attempts to derive an `AccountId` from the origin location
    // using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
    // foreign chains who want to have a local sovereign account on this chain which they control.
    SovereignSignedViaLocation<LocationToAccountId, Origin>,
    // Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
    // recognised.
    RelayChainAsNative<RelayChainOrigin, Origin>,
    // Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
    // recognised.
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
    // Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
    // transaction from the Root origin.
    ParentAsSuperuser<Origin>,
    // Native signed account converter; this just converts an `AccountId32` origin into a normal
    // `Origin::Signed` origin of the same 32-byte value.
    SignedAccountId32AsNative<RelayNetwork, Origin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<Origin>,
);

parameter_types! {
    pub UnitWeightCost: Weight = 1_000;
}

match_type! {
    pub type ParentOrParentsPlurality: impl Contains<MultiLocation> = {
        MultiLocation::X1(Junction::Parent) |
        MultiLocation::X2(Junction::Parent, Junction::Plurality { .. })
    };
}

/// Transparent XcmTransact Barrier for sybil demo. Polkadot will probably come up with a
/// better solution for this. Currently, they have not setup a barrier config for `XcmTransact`
pub struct AllowXcmTransactFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowXcmTransactFrom<T> {
    fn should_execute<Call>(
        _origin: &MultiLocation,
        _top_level: bool,
        message: &Xcm<Call>,
        _shallow_weight: Weight,
        _weight_credit: &mut Weight,
    ) -> Result<(), ()> {
        match message {
            Xcm::Transact {
                origin_type: _,
                require_weight_at_most: _,
                call: _,
            } => Ok(()),
            _ => Err(()),
        }
    }
}

pub struct CrosschainConcreteAsset;
impl FilterAssetLocation for CrosschainConcreteAsset {
    fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
        use xcm::v0::{Junction::*, MultiLocation::*};
        match asset {
            MultiAsset::ConcreteFungible { .. } => match origin {
                Null | X1(Plurality { .. }) => true,
                X1(AccountId32 { .. }) => true,
                X1(Parent { .. }) => true,
                X1(Parachain { .. }) => true,
                X2(Parachain { .. }, _) => true,
                X2(Parent { .. }, _) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

pub type CrosschainAsset = (NativeAsset, CrosschainConcreteAsset);

pub type Barrier = (
    TakeWeightCredit,
    AllowTopLevelPaidExecutionFrom<All<MultiLocation>>,
    AllowUnpaidExecutionFrom<ParentOrParentsPlurality>,
    // ^^^ Parent & its plurality gets free execution
    AllowXcmTransactFrom<All<MultiLocation>>,
);

pub struct XcmConfig;
impl Config for XcmConfig {
    type Call = Call;
    type XcmSender = XcmRouter;
    // How to withdraw and deposit an asset.
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = XcmOriginToTransactDispatchOrigin;
    type IsReserve = MultiNativeAsset; // NativeAsset --> MultiNativeAsset --> CrosschainAsset
    type IsTeleporter = NativeAsset; // <- should be enough to allow teleportation of WND
    type LocationInverter = LocationInverter<Ancestry>;
    type Barrier = ();
    type Weigher = FixedWeightBounds<UnitWeightCost, Call>;
    type Trader = UsingComponents<IdentityFee<Balance>, WestendLocation, AccountId, Balances, ()>;
    type ResponseHandler = (); // Don't handle responses for now.
}

parameter_types! {
    pub const MaxDownwardMessageWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 10;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

pub type XcmRouter = (
    cumulus_primitives_utility::ParentAsUmp<ParachainSystem>,
    XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
    type Event = Event;
    type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
    type XcmRouter = XcmRouter;
    type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
    type XcmExecuteFilter = All<(MultiLocation, Xcm<Call>)>;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = All<(MultiLocation, Vec<MultiAsset>)>;
    type XcmReserveTransferFilter = All<(MultiLocation, Vec<MultiAsset>)>;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call>;
}

impl cumulus_pallet_xcm::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ChannelInfo = ParachainSystem;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
    type Event = Event;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

impl pallet_session::Config for Runtime {
    type Event = Event;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = CollatorSelection;
    // Essentially just Aura, but lets be pedantic.
    type SessionHandler =
        <opaque::SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type Keys = opaque::SessionKeys;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type WeightInfo = weights::pallet_session::WeightInfo<Runtime>;
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
}

parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
    pub const MaxCandidates: u32 = 1000;
    pub const MinCandidates: u32 = 1;
    pub const SessionLength: BlockNumber = 6 * HOURS;
    pub const MaxInvulnerables: u32 = 100;
}

impl pallet_collator_selection::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type PotId = PotId;
    type MaxCandidates = MaxCandidates;
    type MinCandidates = MinCandidates;
    type MaxInvulnerables = MaxInvulnerables;
    // should be a multiple of session or things will get inconsistent
    type KickThreshold = Period;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ValidatorRegistration = Session;
    type WeightInfo = weights::pallet_collator_selection::WeightInfo<Runtime>;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    Call: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = Call;
}

parameter_types! {
    pub const RandomCollectUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}
impl randomness_collect::Config for Runtime {
    type AuthorityId = RandomCollectId;
    type UnsignedPriority = RandomCollectUnsignedPriority;
}

pub type LocalAssetTransactor = CurrencyAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<WestendLocation>,
    // Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
    LocationToAccountId,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We don't track any teleports.
    (),
>;

// orml_xtokens
pub use currency_id_convert::CurrencyIdConvert;

mod currency_id_convert {
    use super::*;
    use xcm::v0::{Junction::*, MultiLocation::*};

    pub struct CurrencyIdConvert;

    const RELAY_CHAIN_CURRENCY_ID: CurrencyId = CurrencyId::WND;

    impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
        fn convert(id: CurrencyId) -> Option<MultiLocation> {
            match id {
                RELAY_CHAIN_CURRENCY_ID => Some(X1(Parent)),
                _ => None,
            }
        }
    }
    impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
        fn convert(location: MultiLocation) -> Option<CurrencyId> {
            match location {
                X1(Parent) => Some(RELAY_CHAIN_CURRENCY_ID),
                _ => None,
            }
        }
    }
    impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
        fn convert(asset: MultiAsset) -> Option<CurrencyId> {
            if let MultiAsset::ConcreteFungible { id, amount: _ } = asset {
                Self::convert(id)
            } else {
                None
            }
        }
    }
}

parameter_types! {
    pub SelfLocation: MultiLocation = MultiLocation::X2(Junction::Parent, Junction::Parachain(ParachainInfo::get().into()));
}

pub struct AccountIdToMultiLocation;

impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
    fn convert(account: AccountId) -> MultiLocation {
        MultiLocation::X1(Junction::AccountId32 {
            network: NetworkId::Any,
            id: account.into(),
        })
    }
}

impl orml_xtokens::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type CurrencyIdConvert = CurrencyIdConvert;
    type AccountIdToMultiLocation = AccountIdToMultiLocation;
    type SelfLocation = SelfLocation;
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type Weigher = FixedWeightBounds<UnitWeightCost, Call>;
    type BaseXcmWeight = UnitWeightCost;
}

// orml_xcm
// impl orml_xcm::Config for Runtime {
//     type Event = Event;
//     type SovereignOrigin = EnsureRoot<AccountId>;
// }

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = jupiter_primitives::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // System support stuff;
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
        ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Config, Storage, Inherent, Event<T>} = 1,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 2,
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 3,
        ParachainInfo: parachain_info::{Pallet, Storage, Config} = 4,
        Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 5,

        // Monetary stuff;
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 6,
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage} = 9,

        // Collator support. the order of these 4 are important and shall not change.
        Authorship: pallet_authorship::{Pallet, Call, Storage} = 20,
        CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 22,
        Aura: pallet_aura::{Pallet, Storage, Config<T>} = 23,
        AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config} = 24,

        // The main stage.
        Utility: pallet_utility::{Pallet, Call, Event} = 30,
        Contracts: pallet_contracts::{Pallet, Call, Storage, Event<T>} = 31,
        RandomnessCollect: randomness_collect::{Pallet, Call, Storage, ValidateUnsigned} = 32,

        // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 40,
        PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin} = 41,
        CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 42,
        DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 43,

        XTokens: orml_xtokens::{Pallet, Storage, Call, Event<T>},
        // OrmlXcm: orml_xcm::{Pallet, Call, Event<T>},
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPallets,
    OnRuntimeUpgrade,
>;

pub struct OnRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for OnRuntimeUpgrade {
    fn on_runtime_upgrade() -> u64 {
        1
    }
}

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }

        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash>
        for Runtime
    {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> ContractExecResult {
            Contracts::bare_call(origin, dest, value, gas_limit, input_data, true)
        }

        fn instantiate(
            origin: AccountId,
            endowment: Balance,
            gas_limit: u64,
            code: pallet_contracts_primitives::Code<Hash>,
            data: Vec<u8>,
            salt: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, BlockNumber>
        {
            Contracts::bare_instantiate(origin, endowment, gas_limit, code, data, salt, true, true)
        }

        fn get_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }

        fn rent_projection(
            address: AccountId,
        ) -> pallet_contracts_primitives::RentProjectionResult<BlockNumber> {
            Contracts::rent_projection(address)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities()
        }
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info() -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info()
        }
    }
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
    fn check_inherents(
        block: &Block,
        relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
    ) -> sp_inherents::CheckInherentsResult {
        let relay_chain_slot = relay_state_proof
            .read_slot()
            .expect("Could not read the relay chain slot from the proof");

        let inherent_data =
            cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
                relay_chain_slot,
                sp_std::time::Duration::from_secs(6),
            )
            .create_inherent_data()
            .expect("Could not create the timestamp inherent data");

        inherent_data.check_extrinsics(&block)
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
    CheckInherents = CheckInherents,
}
