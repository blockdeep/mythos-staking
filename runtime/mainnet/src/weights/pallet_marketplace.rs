
//! Autogenerated weights for `pallet_marketplace`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("mainnet-local-v")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/mythos-node
// benchmark
// pallet
// --chain
// mainnet-local-v
// --pallet
// pallet_marketplace
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --template
// ./.maintain/template.hbs
// --output
// ./runtime/mainnet/src/weights/pallet_marketplace.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_marketplace`.
pub trait WeightInfo {
	fn force_set_authority() -> Weight;
	fn set_fee_signer_address() -> Weight;
	fn set_payout_address() -> Weight;
	fn create_order() -> Weight;
	fn cancel_order() -> Weight;
}

/// Weights for `pallet_marketplace` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Marketplace::Authority` (r:1 w:1)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn force_set_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43`
		//  Estimated: `1505`
		// Minimum execution time: 10_810_000 picoseconds.
		Weight::from_parts(11_190_000, 1505)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::FeeSigner` (r:1 w:1)
	/// Proof: `Marketplace::FeeSigner` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_fee_signer_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `1505`
		// Minimum execution time: 14_510_000 picoseconds.
		Weight::from_parts(15_010_000, 1505)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::PayoutAddress` (r:1 w:1)
	/// Proof: `Marketplace::PayoutAddress` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_payout_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `1505`
		// Minimum execution time: 14_390_000 picoseconds.
		Weight::from_parts(15_440_000, 1505)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(653), added: 3128, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Nonces` (r:1 w:1)
	/// Proof: `Marketplace::Nonces` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::FeeSigner` (r:1 w:0)
	/// Proof: `Marketplace::FeeSigner` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Bids` (r:1 w:1)
	/// Proof: `Marketplace::Bids` (`max_values`: None, `max_size`: Some(172), added: 2647, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Asks` (r:1 w:1)
	/// Proof: `Marketplace::Asks` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::PayoutAddress` (r:1 w:0)
	/// Proof: `Marketplace::PayoutAddress` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:1)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(511), added: 2986, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:2)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(133), added: 2608, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(183), added: 2658, mode: `MaxEncodedLen`)
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1554`
		//  Estimated: `4118`
		// Minimum execution time: 402_432_000 picoseconds.
		Weight::from_parts(410_332_000, 4118)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Bids` (r:1 w:1)
	/// Proof: `Marketplace::Bids` (`max_values`: None, `max_size`: Some(172), added: 2647, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn cancel_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509`
		//  Estimated: `3637`
		// Minimum execution time: 80_131_000 picoseconds.
		Weight::from_parts(81_580_000, 3637)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Marketplace::Authority` (r:1 w:1)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn force_set_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43`
		//  Estimated: `1505`
		// Minimum execution time: 10_810_000 picoseconds.
		Weight::from_parts(11_190_000, 1505)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::FeeSigner` (r:1 w:1)
	/// Proof: `Marketplace::FeeSigner` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_fee_signer_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `1505`
		// Minimum execution time: 14_510_000 picoseconds.
		Weight::from_parts(15_010_000, 1505)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::PayoutAddress` (r:1 w:1)
	/// Proof: `Marketplace::PayoutAddress` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_payout_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `1505`
		// Minimum execution time: 14_390_000 picoseconds.
		Weight::from_parts(15_440_000, 1505)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(653), added: 3128, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Nonces` (r:1 w:1)
	/// Proof: `Marketplace::Nonces` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::FeeSigner` (r:1 w:0)
	/// Proof: `Marketplace::FeeSigner` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Bids` (r:1 w:1)
	/// Proof: `Marketplace::Bids` (`max_values`: None, `max_size`: Some(172), added: 2647, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Asks` (r:1 w:1)
	/// Proof: `Marketplace::Asks` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::PayoutAddress` (r:1 w:0)
	/// Proof: `Marketplace::PayoutAddress` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:1)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(511), added: 2986, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:2)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(133), added: 2608, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(183), added: 2658, mode: `MaxEncodedLen`)
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1554`
		//  Estimated: `4118`
		// Minimum execution time: 402_432_000 picoseconds.
		Weight::from_parts(410_332_000, 4118)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(11_u64))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Bids` (r:1 w:1)
	/// Proof: `Marketplace::Bids` (`max_values`: None, `max_size`: Some(172), added: 2647, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn cancel_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509`
		//  Estimated: `3637`
		// Minimum execution time: 80_131_000 picoseconds.
		Weight::from_parts(81_580_000, 3637)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
