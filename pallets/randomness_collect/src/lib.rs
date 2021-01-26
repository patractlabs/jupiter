//! # Randomness Module
//!
//! The Randomness Collect Module provide a random which collect from relay chain.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, debug, traits::{Randomness as RandomnessT, Get},
	dispatch::DispatchResult, Parameter,
};
use frame_support::sp_runtime::{
	traits::{Hash, Member}, offchain::{http, Duration, storage::StorageValueRef}, RuntimeAppPublic,
	transaction_validity::{
		TransactionSource, TransactionValidity, InvalidTransaction,
		ValidTransaction, TransactionPriority,
	},
	RuntimeDebug,
};

use sp_consensus_vrf::schnorrkel;
use sp_consensus_babe::{VRF_OUTPUT_LENGTH, Epoch};

use frame_system::{
	ensure_root, ensure_none,
	offchain::{SubmitTransaction, SendTransactionTypes},
};

use codec::{Encode, Decode};
use sp_std::prelude::*;
use lite_json::{JsonValue, Serialize};
use hex;
use sp_core::offchain::KeyTypeId;

pub const RAND_COLL: KeyTypeId = KeyTypeId(*b"rand");

pub mod sr25519 {
	mod app_sr25519 {
		use sp_application_crypto::{app_crypto, sr25519};
		use crate::RAND_COLL;
		app_crypto!(sr25519, RAND_COLL);
	}

	sp_application_crypto::with_pair! {
		/// An i'm online keypair using sr25519 as its crypto.
		pub type AuthorityPair = app_sr25519::Pair;
	}

	/// An i'm online signature using sr25519 as its crypto.
	pub type AuthoritySignature = app_sr25519::Signature;

	/// An i'm online identifier using sr25519 as its crypto.
	pub type AuthorityId = app_sr25519::Public;
}

pub mod ed25519 {
	mod app_ed25519 {
		use sp_application_crypto::{app_crypto, ed25519};
		use crate::RAND_COLL;
		app_crypto!(ed25519, RAND_COLL);
	}

	sp_application_crypto::with_pair! {
		/// An i'm online keypair using ed25519 as its crypto.
		pub type AuthorityPair = app_ed25519::Pair;
	}

	/// An i'm online signature using ed25519 as its crypto.
	pub type AuthoritySignature = app_ed25519::Signature;

	/// An i'm online identifier using ed25519 as its crypto.
	pub type AuthorityId = app_ed25519::Public;
}

pub const OCW_DB_RANDOM: &[u8] = b"ocw_collect_random";

/// The rpc port for fetching randomness.
#[derive(Encode, Decode, Clone)]
pub struct RpcPort(pub Vec<u8>);

/// Random which is sent/received.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Random {
	randomness: schnorrkel::Randomness,
	epoch_index: u64,
}

pub trait Config: SendTransactionTypes<Call<Self>> + frame_system::Config {
	type AuthorityId: Member + Parameter + RuntimeAppPublic + Default + Ord;

	type UnsignedPriority: Get<TransactionPriority>;
}

decl_storage! {
	trait Store for Module<T: Config> as RandomnessCollect {
		Key get(fn key): T::AuthorityId;
		/// The `Babe::randomness` from relay chain.
		pub Randomness get(fn randomness): schnorrkel::Randomness;
		/// The `epoch_index` from relay chain.
		EpochIndex get(fn epoch_index): u64;
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
		fn set_key(origin, key: T::AuthorityId) -> DispatchResult {
			ensure_root(origin)?;

			<Key<T>>::put(key);

			Ok(())
		}

		#[weight = 0]
		fn set_randomness(
			origin,
			random: Random,
			_signature: <T::AuthorityId as RuntimeAppPublic>::Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			Randomness::put(random.randomness);
			EpochIndex::put(random.epoch_index);

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			if sp_io::offchain::is_validator() {
				debug::native::info!("randomness collect start.");
				let root_key = Key::<T>::get();
				let mut local_keys = T::AuthorityId::all();
				local_keys.sort();
				local_keys.binary_search(&root_key)
					.ok().map(|location| {
						let signer: T::AuthorityId = local_keys[location].clone();
						let res = Self::fetch_epoch_and_send_signed(signer);
						if let Err(e) = res {
							debug::error!("Error: {}", e);
						} else {
							debug::native::info!("randomness collect success.");
						}
					});
			} else {
				debug::warn!(
					target: "randomness_collect",
					"Skipping collect. Not a validator.",
				)
			}
		}
	}
}

impl<T: Config> Module<T> {
	fn get_rpc_port() -> Option<RpcPort> {
		let storage = StorageValueRef::persistent(OCW_DB_RANDOM);
		let rpc_port: Option<Option<RpcPort>> = storage.get();

		match rpc_port {
			Some(Some(rpc_port)) => {
				Some(rpc_port)
			}
			_ => None,
		}
	}

	fn send_post<'a, 'b>(url: &'a str, post_data: &'a str, response_body: &'b mut Vec<u8>) -> Result<&'b str, http::Error> {
		let request = http::Request::post(url, vec![post_data.as_bytes()]);

		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(1_500));
		let pending = request
			.add_header("Content-Type", "application/json")
			.deadline(deadline)
			.send()
			.map_err(|_| http::Error::IoError)?;

		let response = pending.try_wait(deadline)
			.map_err(|_| http::Error::DeadlineReached)??;
		if response.code != 200 {
			debug::warn!("Unexpected status code: {}", response.code);
			return Err(http::Error::Unknown);
		}

		*response_body = response.body().collect::<Vec<u8>>();
		sp_std::str::from_utf8(response_body).map_err(|_| {
			debug::warn!("No UTF8 body");
			http::Error::Unknown
		})
	}

	fn parse_json_result<'a, 'b>(body: &'a str, buffer: &'b mut Vec<u8>) -> Option<&'b str> {
		let val = lite_json::parse_json(body);

		val.ok().and_then(move |v| match v {
			JsonValue::Object(obj) => {
				obj.into_iter()
					.find(|(k, _)| {
						let mut chars = "result".chars();
						k.iter().all(|k| Some(*k) == chars.next())
					})
					.and_then(move |v| match v.1 {
						JsonValue::String(_) => {
							v.1.serialize_to(buffer, 0, 0);
							sp_std::str::from_utf8(buffer).ok().and_then(|str|
								Some(str.trim_matches('"').trim_start_matches("0x")))
						}
						_ => None,
					})
			}
			_ => None
		})
	}

	fn parse_epoch(body: &str) -> Option<Epoch> {
		let mut buffer = Vec::new();
		let epoch = Self::parse_json_result(body, &mut buffer)?;

		hex::decode(epoch).ok().and_then(
			|epoch_u8| Epoch::decode(&mut epoch_u8.as_slice()).ok())
	}

	fn fetch_epoch(url: &str) -> Result<(u64, schnorrkel::Randomness), http::Error> {
		let fetch_epoch = r#"
			{
				"id": "1",
				"jsonrpc": "2.0",
				"method": "state_call",
				"params": ["BabeApi_current_epoch", "0x"]
			}
		"#;

		let mut epoch_response_body = Vec::new();
		let epoch_response = Self::send_post(url, fetch_epoch, &mut epoch_response_body)?;

		let res = match Self::parse_epoch(epoch_response) {
			Some(epoch) => Ok((epoch.epoch_index, epoch.randomness)),
			None => {
				debug::warn!("Unable to extract epoch from the response: {:?}", epoch_response);
				Err(http::Error::Unknown)
			}
		};
		res
	}

	fn fetch_epoch_and_send_signed(signer: T::AuthorityId) -> Result<(), &'static str> {
		let rpc_port = Self::get_rpc_port();
		if let Some(rpc_port) = rpc_port {
			match sp_std::str::from_utf8(&[b"http://", rpc_port.0.as_slice()].concat()) {
				Ok(rpc_port_str) => {
					let (epoch_index, randomness) = Self::fetch_epoch(rpc_port_str)
						.map_err(|_| "Fetch randomness failed")?;
					let current_epoch_index = Self::epoch_index();
					if epoch_index > current_epoch_index {
						let random = Random{randomness, epoch_index};
						let signature = signer.sign(&random.encode()).ok_or("Failed to sign.")?;
						let call = Call::set_randomness(random, signature);
						SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
							.map_err(|_| "Failed to submit unsigned transaction")?;
					}
				}
				_ => Err("Get rpc_port failed")?,
			}
		}

		Ok(())
	}
}

impl<T: Config> RandomnessT<<T as frame_system::Config>::Hash> for Module<T> {
	fn random(subject: &[u8]) -> T::Hash {
		let mut subject = subject.to_vec();
		subject.reserve(VRF_OUTPUT_LENGTH);
		subject.extend_from_slice(&Self::randomness()[..]);

		<T as frame_system::Config>::Hashing::hash(&subject[..])
	}
}

#[allow(deprecated)]
impl<T: Config> frame_support::unsigned::ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(
		_source: TransactionSource,
		call: &Self::Call,
	) -> TransactionValidity {
		if let Call::set_randomness(ref random, ref signature) = call {
			// check signature
			let key = Key::<T>::get();
			let signature_valid = random.using_encoded(|encoded_random| {
				key.verify(&encoded_random, &signature)
			});
			if !signature_valid {
				return InvalidTransaction::BadProof.into();
			}

			ValidTransaction::with_tag_prefix("RandomnessCollect")
				.priority(T::UnsignedPriority::get())
				.and_provides((random.epoch_index, key))
				.longevity(5)
				.propagate(true)
				.build()
		} else {
			InvalidTransaction::Call.into()
		}
	}
}

