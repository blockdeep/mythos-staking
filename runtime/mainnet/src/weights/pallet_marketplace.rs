
//! Autogenerated weights for `pallet_marketplace`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-04-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `blockdeep-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("mainnet-local-v")`, DB CACHE: 1024

// Executed Command:
// ./target/production/mythos-node
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
// --output
// ./runtime/mainnet/src/weights/pallet_marketplace.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_marketplace`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_marketplace::WeightInfo for WeightInfo<T> {
	/// Storage: `Marketplace::Authority` (r:1 w:1)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn force_set_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `43`
		//  Estimated: `1505`
		// Minimum execution time: 9_850_000 picoseconds.
		Weight::from_parts(10_410_000, 0)
			.saturating_add(Weight::from_parts(0, 1505))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::FeeSigner` (r:1 w:1)
	/// Proof: `Marketplace::FeeSigner` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_fee_signer_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `1505`
		// Minimum execution time: 12_390_000 picoseconds.
		Weight::from_parts(12_970_000, 0)
			.saturating_add(Weight::from_parts(0, 1505))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::PayoutAddress` (r:1 w:1)
	/// Proof: `Marketplace::PayoutAddress` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_payout_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `1505`
		// Minimum execution time: 12_480_000 picoseconds.
		Weight::from_parts(13_320_000, 0)
			.saturating_add(Weight::from_parts(0, 1505))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(637), added: 3112, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Nonces` (r:1 w:1)
	/// Proof: `Marketplace::Nonces` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::FeeSigner` (r:1 w:0)
	/// Proof: `Marketplace::FeeSigner` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Bids` (r:1 w:1)
	/// Proof: `Marketplace::Bids` (`max_values`: None, `max_size`: Some(156), added: 2631, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:2 w:2)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Asks` (r:1 w:1)
	/// Proof: `Marketplace::Asks` (`max_values`: None, `max_size`: Some(161), added: 2636, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::PayoutAddress` (r:1 w:0)
	/// Proof: `Marketplace::PayoutAddress` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Escrow::Deposits` (r:1 w:1)
	/// Proof: `Escrow::Deposits` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:1)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(495), added: 2970, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(169), added: 2644, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(142), added: 2617, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:2)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(151), added: 2626, mode: `MaxEncodedLen`)
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1779`
		//  Estimated: `6266`
		// Minimum execution time: 370_490_000 picoseconds.
		Weight::from_parts(376_180_000, 0)
			.saturating_add(Weight::from_parts(0, 6266))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	/// Storage: `Marketplace::Authority` (r:1 w:0)
	/// Proof: `Marketplace::Authority` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Marketplace::Bids` (r:1 w:1)
	/// Proof: `Marketplace::Bids` (`max_values`: None, `max_size`: Some(156), added: 2631, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn cancel_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493`
		//  Estimated: `3628`
		// Minimum execution time: 64_140_000 picoseconds.
		Weight::from_parts(65_720_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
