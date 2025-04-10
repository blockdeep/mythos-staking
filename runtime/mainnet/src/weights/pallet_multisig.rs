
//! Autogenerated weights for `pallet_multisig`
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
// pallet_multisig
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/mainnet/src/weights/pallet_multisig.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_430_000 picoseconds.
		Weight::from_parts(14_889_756, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(377, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `5587`
		// Minimum execution time: 46_450_000 picoseconds.
		Weight::from_parts(39_406_704, 0)
			.saturating_add(Weight::from_parts(0, 5587))
			// Standard Error: 2_272
			.saturating_add(Weight::from_parts(87_175, 0).saturating_mul(s.into()))
			// Standard Error: 22
			.saturating_add(Weight::from_parts(2_054, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `5587`
		// Minimum execution time: 29_890_000 picoseconds.
		Weight::from_parts(23_768_033, 0)
			.saturating_add(Weight::from_parts(0, 5587))
			// Standard Error: 1_363
			.saturating_add(Weight::from_parts(77_145, 0).saturating_mul(s.into()))
			// Standard Error: 13
			.saturating_add(Weight::from_parts(1_987, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `339 + s * (20 ±0)`
		//  Estimated: `5587`
		// Minimum execution time: 51_511_000 picoseconds.
		Weight::from_parts(41_854_289, 0)
			.saturating_add(Weight::from_parts(0, 5587))
			// Standard Error: 1_314
			.saturating_add(Weight::from_parts(109_392, 0).saturating_mul(s.into()))
			// Standard Error: 12
			.saturating_add(Weight::from_parts(2_080, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `5587`
		// Minimum execution time: 35_560_000 picoseconds.
		Weight::from_parts(37_451_098, 0)
			.saturating_add(Weight::from_parts(0, 5587))
			// Standard Error: 1_688
			.saturating_add(Weight::from_parts(85_556, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `5587`
		// Minimum execution time: 19_891_000 picoseconds.
		Weight::from_parts(20_685_051, 0)
			.saturating_add(Weight::from_parts(0, 5587))
			// Standard Error: 652
			.saturating_add(Weight::from_parts(80_949, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `5587`
		// Minimum execution time: 35_620_000 picoseconds.
		Weight::from_parts(37_022_531, 0)
			.saturating_add(Weight::from_parts(0, 5587))
			// Standard Error: 3_144
			.saturating_add(Weight::from_parts(103_438, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(2122), added: 4597, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn poke_deposit(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `380`
		//  Estimated: `5587`
		// Minimum execution time: 33_750_000 picoseconds.
		Weight::from_parts(35_548_611, 0)
			.saturating_add(Weight::from_parts(0, 5587))
			// Standard Error: 1_339
			.saturating_add(Weight::from_parts(87_919, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
