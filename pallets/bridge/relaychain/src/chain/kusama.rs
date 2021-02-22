use codec::{Decode, Encode};
use sp_runtime::{
    generic,
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};

use crate::chain::{
    BalancesCall, ChargeTransactionPaymentHolder, CheckGenesisHolder, CheckMortalityHolder,
    CheckNonceHolder, CheckSpecVersionHolder, CheckTxVersionHolder, CheckWeightHolder,
    CommonConfig, SystemCall, UtilityCall,
};
use frame_support::weights::RuntimeDbWeight;
use frame_system::limits::{BlockLength, BlockWeights};
use frame_system::{Config, Event, RawOrigin};
use sp_runtime::traits::{AccountIdLookup, StaticLookup};

pub type AccountPublic = <Signature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;
pub type Signature = MultiSignature;
pub type Balance = u128;

pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type SignedExtra = (
    CheckSpecVersionHolder,
    CheckTxVersionHolder,
    CheckGenesisHolder,
    CheckMortalityHolder,
    CheckNonceHolder<u32>,
    CheckWeightHolder,
    ChargeTransactionPaymentHolder<u128>,
);

#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode)]
pub struct KsmRuntime;

#[derive(Encode, Decode)]
pub enum KsmCall<Runtime: CommonConfig + codec::Codec> {
    #[codec(index = 0)]
    System(SystemCall),
    #[codec(index = 4)]
    Balances(BalancesCall<Runtime>),
    #[codec(index = 24)]
    Utility(UtilityCall<Runtime>),
}

impl CommonConfig for KsmRuntime {
    type AccountId = AccountId;
    type Balance = Balance;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Call = KsmCall<Self>;
}

pub type KsmExtrinsic =
    generic::UncheckedExtrinsic<Address, KsmCall<KsmRuntime>, Signature, SignedExtra>;
