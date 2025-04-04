
//! Autogenerated weights for `pallet_collective`
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
// pallet_collective
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: `Council::Members` (r:1 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:100 w:100)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (2021 ±0) + p * (2026 ±0)`
		//  Estimated: `12163 + m * (1231 ±15) + p * (3660 ±15)`
		// Minimum execution time: 17_360_000 picoseconds.
		Weight::from_parts(17_600_000, 0)
			.saturating_add(Weight::from_parts(0, 12163))
			// Standard Error: 76_814
			.saturating_add(Weight::from_parts(5_747_991, 0).saturating_mul(m.into()))
			// Standard Error: 76_814
			.saturating_add(Weight::from_parts(10_558_297, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1231).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 3660).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (20 ±0)`
		//  Estimated: `1517 + m * (20 ±0)`
		// Minimum execution time: 15_980_000 picoseconds.
		Weight::from_parts(15_846_958, 0)
			.saturating_add(Weight::from_parts(0, 1517))
			// Standard Error: 30
			.saturating_add(Weight::from_parts(1_411, 0).saturating_mul(b.into()))
			// Standard Error: 312
			.saturating_add(Weight::from_parts(12_620, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 20).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (20 ±0)`
		//  Estimated: `3497 + m * (20 ±0)`
		// Minimum execution time: 19_490_000 picoseconds.
		Weight::from_parts(19_299_592, 0)
			.saturating_add(Weight::from_parts(0, 3497))
			// Standard Error: 35
			.saturating_add(Weight::from_parts(1_389, 0).saturating_mul(b.into()))
			// Standard Error: 368
			.saturating_add(Weight::from_parts(18_788, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 20).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `320 + m * (20 ±0) + p * (36 ±0)`
		//  Estimated: `3714 + m * (21 ±0) + p * (36 ±0)`
		// Minimum execution time: 25_370_000 picoseconds.
		Weight::from_parts(27_834_644, 0)
			.saturating_add(Weight::from_parts(0, 3714))
			// Standard Error: 149
			.saturating_add(Weight::from_parts(1_617, 0).saturating_mul(b.into()))
			// Standard Error: 1_558
			.saturating_add(Weight::from_parts(26_008, 0).saturating_mul(m.into()))
			// Standard Error: 1_538
			.saturating_add(Weight::from_parts(178_702, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 21).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `795 + m * (40 ±0)`
		//  Estimated: `4259 + m * (40 ±0)`
		// Minimum execution time: 25_880_000 picoseconds.
		Weight::from_parts(26_292_269, 0)
			.saturating_add(Weight::from_parts(0, 4259))
			// Standard Error: 1_859
			.saturating_add(Weight::from_parts(43_748, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `372 + m * (40 ±0) + p * (36 ±0)`
		//  Estimated: `3817 + m * (41 ±0) + p * (36 ±0)`
		// Minimum execution time: 28_900_000 picoseconds.
		Weight::from_parts(30_752_356, 0)
			.saturating_add(Weight::from_parts(0, 3817))
			// Standard Error: 2_400
			.saturating_add(Weight::from_parts(6_727, 0).saturating_mul(m.into()))
			// Standard Error: 2_340
			.saturating_add(Weight::from_parts(221_100, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 41).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `674 + b * (1 ±0) + m * (40 ±0) + p * (40 ±0)`
		//  Estimated: `3991 + b * (1 ±0) + m * (42 ±0) + p * (40 ±0)`
		// Minimum execution time: 41_001_000 picoseconds.
		Weight::from_parts(47_001_144, 0)
			.saturating_add(Weight::from_parts(0, 3991))
			// Standard Error: 191
			.saturating_add(Weight::from_parts(1_961, 0).saturating_mul(b.into()))
			// Standard Error: 2_028
			.saturating_add(Weight::from_parts(21_579, 0).saturating_mul(m.into()))
			// Standard Error: 1_977
			.saturating_add(Weight::from_parts(203_039, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 42).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392 + m * (40 ±0) + p * (36 ±0)`
		//  Estimated: `3837 + m * (41 ±0) + p * (36 ±0)`
		// Minimum execution time: 32_090_000 picoseconds.
		Weight::from_parts(34_131_297, 0)
			.saturating_add(Weight::from_parts(0, 3837))
			// Standard Error: 2_109
			.saturating_add(Weight::from_parts(37_329, 0).saturating_mul(m.into()))
			// Standard Error: 2_057
			.saturating_add(Weight::from_parts(202_835, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 41).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `694 + b * (1 ±0) + m * (40 ±0) + p * (40 ±0)`
		//  Estimated: `4011 + b * (1 ±0) + m * (42 ±0) + p * (40 ±0)`
		// Minimum execution time: 48_620_000 picoseconds.
		Weight::from_parts(48_125_032, 0)
			.saturating_add(Weight::from_parts(0, 4011))
			// Standard Error: 195
			.saturating_add(Weight::from_parts(2_287, 0).saturating_mul(b.into()))
			// Standard Error: 2_063
			.saturating_add(Weight::from_parts(35_024, 0).saturating_mul(m.into()))
			// Standard Error: 2_011
			.saturating_add(Weight::from_parts(208_612, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 42).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189 + p * (32 ±0)`
		//  Estimated: `1674 + p * (32 ±0)`
		// Minimum execution time: 16_470_000 picoseconds.
		Weight::from_parts(18_433_454, 0)
			.saturating_add(Weight::from_parts(0, 1674))
			// Standard Error: 1_524
			.saturating_add(Weight::from_parts(144_480, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:1 w:0)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `d` is `[0, 1]`.
	/// The range of component `p` is `[1, 100]`.
	fn kill(d: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1460 + p * (36 ±0)`
		//  Estimated: `4859 + d * (123 ±6) + p * (37 ±0)`
		// Minimum execution time: 23_450_000 picoseconds.
		Weight::from_parts(26_900_289, 0)
			.saturating_add(Weight::from_parts(0, 4859))
			// Standard Error: 69_048
			.saturating_add(Weight::from_parts(331_880, 0).saturating_mul(d.into()))
			// Standard Error: 1_069
			.saturating_add(Weight::from_parts(178_878, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 123).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(p.into()))
	}
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:1 w:0)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn release_proposal_cost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `874`
		//  Estimated: `4339`
		// Minimum execution time: 16_620_000 picoseconds.
		Weight::from_parts(17_280_000, 0)
			.saturating_add(Weight::from_parts(0, 4339))
			.saturating_add(T::DbWeight::get().reads(2))
	}
}
