
//! Autogenerated weights for `pallet_collator_staking`
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
// pallet_collator_staking
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/mainnet/src/weights/pallet_collator_staking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collator_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_staking::WeightInfo for WeightInfo<T> {
	/// Storage: `CollatorStaking::Candidates` (r:4 w:0)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:4 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorStaking::CounterForCandidates` (r:1 w:0)
	/// Proof: `CollatorStaking::CounterForCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Invulnerables` (r:0 w:1)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[2, 4]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427 + b * (64 ±0)`
		//  Estimated: `1489 + b * (2539 ±0)`
		// Minimum execution time: 33_130_000 picoseconds.
		Weight::from_parts(22_018_950, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 16_913
			.saturating_add(Weight::from_parts(6_053_723, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2539).saturating_mul(b.into()))
	}
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorStaking::Candidates` (r:1 w:0)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Invulnerables` (r:1 w:1)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 3]`.
	fn add_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467 + b * (22 ±0)`
		//  Estimated: `3928 + b * (25 ±0)`
		// Minimum execution time: 23_990_000 picoseconds.
		Weight::from_parts(24_287_967, 0)
			.saturating_add(Weight::from_parts(0, 3928))
			// Standard Error: 15_304
			.saturating_add(Weight::from_parts(677_248, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 25).saturating_mul(b.into()))
	}
	/// Storage: `CollatorStaking::CounterForCandidates` (r:1 w:0)
	/// Proof: `CollatorStaking::CounterForCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Invulnerables` (r:1 w:1)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[3, 4]`.
	fn remove_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208 + b * (21 ±0)`
		//  Estimated: `1566`
		// Minimum execution time: 16_140_000 picoseconds.
		Weight::from_parts(15_088_332, 0)
			.saturating_add(Weight::from_parts(0, 1566))
			// Standard Error: 35_789
			.saturating_add(Weight::from_parts(540_691, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::DesiredCandidates` (r:0 w:1)
	/// Proof: `CollatorStaking::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `1566`
		// Minimum execution time: 12_290_000 picoseconds.
		Weight::from_parts(12_690_000, 0)
			.saturating_add(Weight::from_parts(0, 1566))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::MinCandidacyBond` (r:0 w:1)
	/// Proof: `CollatorStaking::MinCandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_min_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_720_000 picoseconds.
		Weight::from_parts(7_060_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::CounterForCandidates` (r:1 w:1)
	/// Proof: `CollatorStaking::CounterForCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorStaking::MinCandidacyBond` (r:1 w:0)
	/// Proof: `CollatorStaking::MinCandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:1 w:1)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidateStake` (r:1 w:0)
	/// Proof: `CollatorStaking::CandidateStake` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidacyBondReleases` (r:1 w:1)
	/// Proof: `CollatorStaking::CandidacyBondReleases` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::SessionRemovedCandidates` (r:0 w:1)
	/// Proof: `CollatorStaking::SessionRemovedCandidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorStaking::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn register_as_candidate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `563`
		//  Estimated: `4752`
		// Minimum execution time: 74_200_000 picoseconds.
		Weight::from_parts(74_931_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `CollatorStaking::Candidates` (r:16 w:1)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:15 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidacyBondReleases` (r:1 w:1)
	/// Proof: `CollatorStaking::CandidacyBondReleases` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CounterForCandidates` (r:1 w:1)
	/// Proof: `CollatorStaking::CounterForCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::SessionRemovedCandidates` (r:0 w:1)
	/// Proof: `CollatorStaking::SessionRemovedCandidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn remove_worst_candidate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2262`
		//  Estimated: `52170`
		// Minimum execution time: 202_491_000 picoseconds.
		Weight::from_parts(203_990_000, 0)
			.saturating_add(Weight::from_parts(0, 52170))
			.saturating_add(T::DbWeight::get().reads(35))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `CollatorStaking::CounterForCandidates` (r:1 w:1)
	/// Proof: `CollatorStaking::CounterForCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:1 w:1)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidacyBondReleases` (r:1 w:1)
	/// Proof: `CollatorStaking::CandidacyBondReleases` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::SessionRemovedCandidates` (r:0 w:1)
	/// Proof: `CollatorStaking::SessionRemovedCandidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorStaking::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn leave_intent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `571`
		//  Estimated: `4752`
		// Minimum execution time: 80_220_000 picoseconds.
		Weight::from_parts(81_720_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `CollatorStaking::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::ProducedBlocks` (r:1 w:1)
	/// Proof: `CollatorStaking::ProducedBlocks` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::TotalBlocks` (r:1 w:1)
	/// Proof: `CollatorStaking::TotalBlocks` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorStaking::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `248`
		//  Estimated: `3505`
		// Minimum execution time: 12_990_000 picoseconds.
		Weight::from_parts(13_500_000, 0)
			.saturating_add(Weight::from_parts(0, 3505))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `CollatorStaking::CounterForCandidates` (r:1 w:0)
	/// Proof: `CollatorStaking::CounterForCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::MinCandidacyBond` (r:1 w:0)
	/// Proof: `CollatorStaking::MinCandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:16 w:0)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::LastAuthoredBlock` (r:15 w:0)
	/// Proof: `CollatorStaking::LastAuthoredBlock` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:15 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorStaking::Invulnerables` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::DesiredCandidates` (r:1 w:0)
	/// Proof: `CollatorStaking::DesiredCandidates` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidacyBondReleases` (r:14 w:14)
	/// Proof: `CollatorStaking::CandidacyBondReleases` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:14 w:14)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:14 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::SessionRemovedCandidates` (r:0 w:14)
	/// Proof: `CollatorStaking::SessionRemovedCandidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 15]`.
	/// The range of component `c` is `[1, 15]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `327 + c * (159 ±0) + r * (114 ±0)`
		//  Estimated: `4752 + c * (3412 ±0) + r * (3762 ±0)`
		// Minimum execution time: 46_330_000 picoseconds.
		Weight::from_parts(46_890_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			// Standard Error: 806_437
			.saturating_add(Weight::from_parts(39_444_181, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 3412).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 3762).saturating_mul(r.into()))
	}
	/// Storage: `CollatorStaking::UserStake` (r:1 w:1)
	/// Proof: `CollatorStaking::UserStake` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CurrentSession` (r:1 w:0)
	/// Proof: `CollatorStaking::CurrentSession` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:3 w:3)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidateStake` (r:3 w:3)
	/// Proof: `CollatorStaking::CandidateStake` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::MinStake` (r:1 w:0)
	/// Proof: `CollatorStaking::MinStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	fn stake(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `403 + c * (99 ±0)`
		//  Estimated: `4402 + c * (2567 ±0)`
		// Minimum execution time: 43_570_000 picoseconds.
		Weight::from_parts(25_302_782, 0)
			.saturating_add(Weight::from_parts(0, 4402))
			// Standard Error: 25_429
			.saturating_add(Weight::from_parts(19_602_560, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2567).saturating_mul(c.into()))
	}
	/// Storage: `CollatorStaking::UserStake` (r:1 w:1)
	/// Proof: `CollatorStaking::UserStake` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CurrentSession` (r:1 w:0)
	/// Proof: `CollatorStaking::CurrentSession` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidateStake` (r:1 w:1)
	/// Proof: `CollatorStaking::CandidateStake` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:1 w:1)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn unstake_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `608`
		//  Estimated: `3604`
		// Minimum execution time: 42_390_000 picoseconds.
		Weight::from_parts(42_950_000, 0)
			.saturating_add(Weight::from_parts(0, 3604))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `CollatorStaking::UserStake` (r:1 w:1)
	/// Proof: `CollatorStaking::UserStake` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CurrentSession` (r:1 w:0)
	/// Proof: `CollatorStaking::CurrentSession` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidateStake` (r:3 w:3)
	/// Proof: `CollatorStaking::CandidateStake` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:3 w:3)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	fn unstake_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `420 + c * (188 ±0)`
		//  Estimated: `3604 + c * (2567 ±0)`
		// Minimum execution time: 41_670_000 picoseconds.
		Weight::from_parts(24_639_359, 0)
			.saturating_add(Weight::from_parts(0, 3604))
			// Standard Error: 70_671
			.saturating_add(Weight::from_parts(18_535_471, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2567).saturating_mul(c.into()))
	}
	/// Storage: `CollatorStaking::CandidacyBondReleases` (r:1 w:1)
	/// Proof: `CollatorStaking::CandidacyBondReleases` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::ReleaseQueues` (r:1 w:1)
	/// Proof: `CollatorStaking::ReleaseQueues` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	fn release(_c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `577 + c * (21 ±0)`
		//  Estimated: `4752`
		// Minimum execution time: 62_611_000 picoseconds.
		Weight::from_parts(64_687_975, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `CollatorStaking::UserStake` (r:1 w:1)
	/// Proof: `CollatorStaking::UserStake` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CurrentSession` (r:1 w:0)
	/// Proof: `CollatorStaking::CurrentSession` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CandidateStake` (r:3 w:3)
	/// Proof: `CollatorStaking::CandidateStake` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::PerSessionRewards` (r:365 w:365)
	/// Proof: `CollatorStaking::PerSessionRewards` (`max_values`: None, `max_size`: Some(833), added: 3308, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::ClaimableRewards` (r:1 w:1)
	/// Proof: `CollatorStaking::ClaimableRewards` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::AutoCompound` (r:1 w:0)
	/// Proof: `CollatorStaking::AutoCompound` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:3 w:3)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::MinStake` (r:1 w:0)
	/// Proof: `CollatorStaking::MinStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	/// The range of component `r` is `[1, 365]`.
	fn claim_rewards(c: u32, r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1700 + c * (151 ±0) + r * (840 ±0)`
		//  Estimated: `6172 + c * (2567 ±35) + r * (3303 ±0)`
		// Minimum execution time: 208_951_000 picoseconds.
		Weight::from_parts(210_591_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			// Standard Error: 1_027_711
			.saturating_add(Weight::from_parts(17_877_995, 0).saturating_mul(c.into()))
			// Standard Error: 8_295
			.saturating_add(Weight::from_parts(7_157_207, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2567).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 3303).saturating_mul(r.into()))
	}
	/// Storage: `CollatorStaking::UserStake` (r:1 w:0)
	/// Proof: `CollatorStaking::UserStake` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CurrentSession` (r:1 w:0)
	/// Proof: `CollatorStaking::CurrentSession` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::AutoCompound` (r:0 w:1)
	/// Proof: `CollatorStaking::AutoCompound` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	fn set_autocompound_percentage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `4402`
		// Minimum execution time: 26_080_000 picoseconds.
		Weight::from_parts(26_481_000, 0)
			.saturating_add(Weight::from_parts(0, 4402))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::CollatorRewardPercentage` (r:0 w:1)
	/// Proof: `CollatorStaking::CollatorRewardPercentage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn set_collator_reward_percentage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_580_000 picoseconds.
		Weight::from_parts(6_860_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::ExtraReward` (r:0 w:1)
	/// Proof: `CollatorStaking::ExtraReward` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_extra_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_680_000 picoseconds.
		Weight::from_parts(7_070_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::MinCandidacyBond` (r:1 w:0)
	/// Proof: `CollatorStaking::MinCandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::MinStake` (r:0 w:1)
	/// Proof: `CollatorStaking::MinStake` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_minimum_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `267`
		//  Estimated: `1501`
		// Minimum execution time: 13_561_000 picoseconds.
		Weight::from_parts(14_160_000, 0)
			.saturating_add(Weight::from_parts(0, 1501))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::ExtraReward` (r:1 w:1)
	/// Proof: `CollatorStaking::ExtraReward` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn stop_extra_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `440`
		//  Estimated: `6172`
		// Minimum execution time: 66_650_000 picoseconds.
		Weight::from_parts(67_220_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn top_up_extra_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3581`
		// Minimum execution time: 48_690_000 picoseconds.
		Weight::from_parts(49_440_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::TotalBlocks` (r:0 w:1)
	/// Proof: `CollatorStaking::TotalBlocks` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CurrentSession` (r:0 w:1)
	/// Proof: `CollatorStaking::CurrentSession` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn start_session() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_330_000 picoseconds.
		Weight::from_parts(2_380_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `CollatorStaking::ExtraReward` (r:1 w:0)
	/// Proof: `CollatorStaking::ExtraReward` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CounterForPerSessionRewards` (r:1 w:1)
	/// Proof: `CollatorStaking::CounterForPerSessionRewards` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::ClaimableRewards` (r:1 w:1)
	/// Proof: `CollatorStaking::ClaimableRewards` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:16 w:16)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::TotalBlocks` (r:1 w:0)
	/// Proof: `CollatorStaking::TotalBlocks` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::CollatorRewardPercentage` (r:1 w:0)
	/// Proof: `CollatorStaking::CollatorRewardPercentage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::ProducedBlocks` (r:16 w:15)
	/// Proof: `CollatorStaking::ProducedBlocks` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:15 w:0)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::PerSessionRewards` (r:1 w:1)
	/// Proof: `CollatorStaking::PerSessionRewards` (`max_values`: None, `max_size`: Some(833), added: 3308, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 15]`.
	fn end_session(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429 + c * (265 ±0)`
		//  Estimated: `4298 + c * (2591 ±0)`
		// Minimum execution time: 89_870_000 picoseconds.
		Weight::from_parts(41_701_608, 0)
			.saturating_add(Weight::from_parts(0, 4298))
			// Standard Error: 35_105
			.saturating_add(Weight::from_parts(51_326_476, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2591).saturating_mul(c.into()))
	}
	/// Storage: `CollatorStaking::MinCandidacyBond` (r:1 w:0)
	/// Proof: `CollatorStaking::MinCandidacyBond` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::Candidates` (r:1 w:0)
	/// Proof: `CollatorStaking::Candidates` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	fn update_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `514`
		//  Estimated: `4752`
		// Minimum execution time: 50_360_000 picoseconds.
		Weight::from_parts(51_200_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	fn lock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `4752`
		// Minimum execution time: 35_130_000 picoseconds.
		Weight::from_parts(35_630_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `CollatorStaking::UserStake` (r:1 w:1)
	/// Proof: `CollatorStaking::UserStake` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::ReleaseQueues` (r:1 w:1)
	/// Proof: `CollatorStaking::ReleaseQueues` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
	/// Storage: `CollatorStaking::AutoCompound` (r:1 w:1)
	/// Proof: `CollatorStaking::AutoCompound` (`max_values`: None, `max_size`: Some(37), added: 2512, mode: `MaxEncodedLen`)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `4752`
		// Minimum execution time: 60_930_000 picoseconds.
		Weight::from_parts(61_900_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
