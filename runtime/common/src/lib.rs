#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight};
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{Pair, Public, U256};

use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

pub use account::EthereumSignature;
use frame_support::traits::Incrementable;

// Cumulus types re-export
// These types are shared between the mainnet and testnet runtimes
//https://github.com/paritytech/cumulus/tree/master/parachains/common
pub use parachains_common::{AuraId, Balance, Block, BlockNumber, Hash};

extern crate alloc;

pub type Signature = EthereumSignature;

/// Use AccountId20 for Ethereum address
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub type AccountIdOf<R> = <R as frame_system::Config>::AccountId;

/// Nonce for an account
pub type Nonce = u32;

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
// Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(100);

/// Max block weight configuration, max allowed pov size is 10 MiB on all relay-chains.
pub const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), 10 * 1024 * 1024 as u64);

#[derive(
	Clone,
	TypeInfo,
	Encode,
	PartialEq,
	Eq,
	Decode,
	DecodeWithMemTracking,
	Copy,
	MaxEncodedLen,
	Debug,
)]
pub struct IncrementableU256(U256);

impl Incrementable for IncrementableU256 {
	fn increment(&self) -> Option<Self> {
		self.0.checked_add(U256::one()).map_or_else(|| None, |value| Some(Self(value)))
	}

	fn initial_value() -> Option<Self> {
		Some(Self(U256::zero()))
	}
}

//Needed for Pallet Nfts Benchmark Helper
impl From<u16> for IncrementableU256 {
	fn from(value: u16) -> Self {
		IncrementableU256(U256::from(value))
	}
}

/// The default XCM version to set in genesis config.
pub const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&alloc::format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <EthereumSignature as Verify>::Signer;

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}
