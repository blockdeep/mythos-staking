
//! Autogenerated weights for `pallet_treasury`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.0.0
//! DATE: 2025-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// pallet_treasury
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/mainnet/src/weights/pallet_treasury.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	/// Storage: `Treasury::ProposalCount` (r:1 w:1)
	/// Proof: `Treasury::ProposalCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Proposals` (r:0 w:1)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn spend_local() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1887`
		// Minimum execution time: 14_100_000 picoseconds.
		Weight::from_parts(14_580_000, 0)
			.saturating_add(Weight::from_parts(0, 1887))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	fn remove_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `90`
		//  Estimated: `1887`
		// Minimum execution time: 7_770_000 picoseconds.
		Weight::from_parts(7_940_000, 0)
			.saturating_add(Weight::from_parts(0, 1887))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Deactivated` (r:1 w:1)
	/// Proof: `Treasury::Deactivated` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::LastSpendPeriod` (r:1 w:1)
	/// Proof: `Treasury::LastSpendPeriod` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 99]`.
	fn on_initialize_proposals(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `192 + p * (1 ±0)`
		//  Estimated: `3581`
		// Minimum execution time: 15_550_000 picoseconds.
		Weight::from_parts(18_207_908, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			// Standard Error: 753
			.saturating_add(Weight::from_parts(23_363, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Treasury::SpendCount` (r:1 w:1)
	/// Proof: `Treasury::SpendCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Spends` (r:0 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1489`
		// Minimum execution time: 12_600_000 picoseconds.
		Weight::from_parts(13_050_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `368`
		//  Estimated: `6172`
		// Minimum execution time: 59_120_000 picoseconds.
		Weight::from_parts(59_600_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn check_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `112`
		//  Estimated: `3522`
		// Minimum execution time: 14_480_000 picoseconds.
		Weight::from_parts(15_230_000, 0)
			.saturating_add(Weight::from_parts(0, 3522))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn void_spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `112`
		//  Estimated: `3522`
		// Minimum execution time: 13_740_000 picoseconds.
		Weight::from_parts(14_260_000, 0)
			.saturating_add(Weight::from_parts(0, 3522))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
