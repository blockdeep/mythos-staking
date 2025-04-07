
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.0.0
//! DATE: 2025-03-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_utility
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_540_000 picoseconds.
		Weight::from_parts(6_740_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 3_317
			.saturating_add(Weight::from_parts(3_847_775, 0).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_830_000 picoseconds.
		Weight::from_parts(6_060_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_150_000 picoseconds.
		Weight::from_parts(6_330_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 12_230
			.saturating_add(Weight::from_parts(4_162_149, 0).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_470_000 picoseconds.
		Weight::from_parts(8_790_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_300_000 picoseconds.
		Weight::from_parts(6_440_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 3_603
			.saturating_add(Weight::from_parts(3_839_398, 0).saturating_mul(c.into()))
	}
	fn dispatch_as_fallible() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_417_000 picoseconds.
		Weight::from_parts(5_705_000, 0)
	}
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:2 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	fn if_else() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `7004`
		// Minimum execution time: 11_273_000 picoseconds.
		Weight::from_parts(11_571_000, 7004)
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
}
