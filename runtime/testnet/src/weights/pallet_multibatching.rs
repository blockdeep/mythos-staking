
//! Autogenerated weights for `pallet_multibatching`
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
// pallet_multibatching
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_multibatching.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multibatching`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multibatching::WeightInfo for WeightInfo<T> {
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 128]`.
	/// The range of component `s` is `[1, 128]`.
	fn batch(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `118`
		//  Estimated: `3497`
		// Minimum execution time: 368_950_000 picoseconds.
		Weight::from_parts(375_291_000, 0)
			.saturating_add(Weight::from_parts(0, 3497))
			// Standard Error: 206_680
			.saturating_add(Weight::from_parts(6_247_007, 0).saturating_mul(c.into()))
			// Standard Error: 206_680
			.saturating_add(Weight::from_parts(59_829_603, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 128]`.
	/// The range of component `s` is `[1, 128]`.
	fn batch_v2(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `118`
		//  Estimated: `3497`
		// Minimum execution time: 370_460_000 picoseconds.
		Weight::from_parts(372_830_000, 0)
			.saturating_add(Weight::from_parts(0, 3497))
			// Standard Error: 205_959
			.saturating_add(Weight::from_parts(6_211_654, 0).saturating_mul(c.into()))
			// Standard Error: 205_959
			.saturating_add(Weight::from_parts(59_901_133, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
