pub mod kusama;

use codec::{Codec, Decode, Encode};

use frame_support::Parameter;
use frame_system::RawOrigin;

use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeDisplay, MaybeSerializeDeserialize, Member, StaticLookup, SignedExtension, AtLeast32Bit};
use sp_std::{fmt::Debug, prelude::*};
use frame_support::pallet_prelude::TransactionValidityError;
use sp_runtime::generic::Era;

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckSpecVersionHolder;

impl SignedExtension for CheckSpecVersionHolder {
    const IDENTIFIER: &'static str = "";
    type AccountId = ();
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckTxVersionHolder;
impl SignedExtension for CheckTxVersionHolder {
    const IDENTIFIER: &'static str = "";
    type AccountId = ();
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckGenesisHolder;
impl SignedExtension for CheckGenesisHolder {
    const IDENTIFIER: &'static str = "";
    type AccountId = ();
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckMortalityHolder(Era);
impl SignedExtension for CheckMortalityHolder {
    const IDENTIFIER: &'static str = "";
    type AccountId = ();
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct CheckNonceHolder<Index: Parameter + Member + MaybeSerializeDeserialize + Debug + Default + MaybeDisplay + AtLeast32Bit
+ Copy>(#[codec(compact)] Index);
impl<Index: Parameter + Member + MaybeSerializeDeserialize + Debug + Default + MaybeDisplay + AtLeast32Bit
+ Copy> SignedExtension for CheckNonceHolder<Index> {
    const IDENTIFIER: &'static str = "";
    type AccountId = ();
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, Default, Debug)]
pub struct CheckWeightHolder;
impl SignedExtension for CheckWeightHolder {
    const IDENTIFIER: &'static str = "";
    type AccountId = ();
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
pub struct ChargeTransactionPaymentHolder<Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
MaybeSerializeDeserialize + Debug>(#[codec(compact)] Balance);

impl<Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + Default + Copy +
MaybeSerializeDeserialize + Debug> SignedExtension for ChargeTransactionPaymentHolder<Balance> {
    const IDENTIFIER: &'static str = "";
    type AccountId = ();
    type Call = ();
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }
}

pub trait CommonConfig: 'static + Eq + Clone {
    /// The user account identifier type for the runtime.
    type AccountId: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Debug
        + MaybeDisplay
        + Ord
        + Default;

    type Balance: Parameter
        + Member
        + AtLeast32BitUnsigned
        + Codec
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + Debug;

    type Lookup: StaticLookup<Target = Self::AccountId>;

    type Call: Decode; //: Codec;
}

#[allow(non_camel_case_types)]
#[derive(Encode, Decode)]
pub enum BalancesCall<T: CommonConfig + Codec> {
    transfer(
        <T::Lookup as StaticLookup>::Source,
        #[codec(compact)] T::Balance,
    ),
    set_balance(
        <T::Lookup as StaticLookup>::Source,
        #[codec(compact)] T::Balance,
        #[codec(compact)] T::Balance,
    ),
    force_transfer(
        <T::Lookup as StaticLookup>::Source,
        <T::Lookup as StaticLookup>::Source,
        #[codec(compact)] T::Balance,
    ),
    transfer_keep_alive(
        <T::Lookup as StaticLookup>::Source,
        #[codec(compact)] T::Balance,
    ),
}

#[allow(non_camel_case_types)]
#[derive(Encode, Decode)]
pub enum SystemCall {
    fill_block,
    remark(Vec<u8>),
    set_heap_pages,
    set_code,
    set_code_without_checks,
    set_changes_trie_config,
    set_storage,
    kill_storage,
    kill_prefix,
}

#[allow(non_camel_case_types)]
#[derive(Encode)]
pub enum UtilityCall<T: CommonConfig + Codec> {
    batch,
    as_derivative,
    // only allow this batch
    batch_all(Vec<T::Call>),
}

impl<R: CommonConfig + Codec> Decode for UtilityCall<R> {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let index: u8 = Decode::decode(input)?;
        let r = match index {
            0 => UtilityCall::batch,
            1 => UtilityCall::as_derivative,
            2 => UtilityCall::batch_all(Decode::decode(input)?),
            _ => Err(codec::Error::from("Unexpect enum index."))?,
        };
        Ok(r)
    }
}
