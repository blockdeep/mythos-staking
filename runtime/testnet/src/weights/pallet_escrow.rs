
//! Autogenerated weights for `pallet_escrow`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-04-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `blockdeep-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local-v")`, DB CACHE: 1024

// Executed Command:
// ./target/production/mythos-node
// benchmark
// pallet
// --chain
// local-v
// --pallet
// pallet_escrow
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_escrow.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_escrow`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_escrow::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Escrow::Deposits` (r:1 w:1)
	/// Proof: `Escrow::Deposits` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `200`
		//  Estimated: `3628`
		// Minimum execution time: 89_660_000 picoseconds.
		Weight::from_parts(91_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Escrow::Deposits` (r:1 w:1)
	/// Proof: `Escrow::Deposits` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn release() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3628`
		// Minimum execution time: 50_320_000 picoseconds.
		Weight::from_parts(51_110_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Escrow::Deposits` (r:1 w:1)
	/// Proof: `Escrow::Deposits` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn revoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3628`
		// Minimum execution time: 82_440_000 picoseconds.
		Weight::from_parts(83_220_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Escrow::Deposits` (r:1 w:1)
	/// Proof: `Escrow::Deposits` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	fn force_release() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3628`
		// Minimum execution time: 50_160_000 picoseconds.
		Weight::from_parts(51_060_000, 0)
			.saturating_add(Weight::from_parts(0, 3628))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
