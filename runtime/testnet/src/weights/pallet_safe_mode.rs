
//! Autogenerated weights for `pallet_safe_mode`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.0.0
//! DATE: 2025-03-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `blockdeep-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local-v")`, DB CACHE: 1024

// Executed Command:
// ./target/release/mythos-node
// benchmark
// pallet
// --chain
// local-v
// --pallet
// pallet_safe_mode
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_safe_mode.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_safe_mode`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_safe_mode::WeightInfo for WeightInfo<T> {
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1489`
		// Minimum execution time: 4_030_000 picoseconds.
		Weight::from_parts(4_110_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn on_initialize_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `1489`
		// Minimum execution time: 11_980_000 picoseconds.
		Weight::from_parts(12_620_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(0, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1489`
		// Minimum execution time: 14_770_000 picoseconds.
		Weight::from_parts(15_250_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::Deposits` (r:0 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3628`
		// Minimum execution time: 85_090_000 picoseconds.
		Weight::from_parts(85_770_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `1489`
		// Minimum execution time: 15_700_000 picoseconds.
		Weight::from_parts(16_070_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:1)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn force_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `1489`
		// Minimum execution time: 15_340_000 picoseconds.
		Weight::from_parts(15_910_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn release_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `3628`
		// Minimum execution time: 69_490_000 picoseconds.
		Weight::from_parts(70_560_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn force_release_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `3628`
		// Minimum execution time: 66_610_000 picoseconds.
		Weight::from_parts(67_700_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `SafeMode::Deposits` (r:1 w:1)
	/// Proof: `SafeMode::Deposits` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn force_slash_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `3628`
		// Minimum execution time: 53_410_000 picoseconds.
		Weight::from_parts(54_280_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
