
//! Autogenerated weights for `pallet_democracy`
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
// pallet_democracy
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/mainnet/src/weights/pallet_democracy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	/// Storage: `Democracy::PublicPropCount` (r:1 w:1)
	/// Proof: `Democracy::PublicPropCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(15502), added: 15997, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(2038), added: 4513, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:0 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(2030), added: 4505, mode: `MaxEncodedLen`)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3810`
		//  Estimated: `16987`
		// Minimum execution time: 48_641_000 picoseconds.
		Weight::from_parts(49_471_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(2030), added: 4505, mode: `MaxEncodedLen`)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2526`
		//  Estimated: `5495`
		// Minimum execution time: 43_750_000 picoseconds.
		Weight::from_parts(44_430_000, 0)
			.saturating_add(Weight::from_parts(0, 5495))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3369`
		//  Estimated: `7248`
		// Minimum execution time: 64_291_000 picoseconds.
		Weight::from_parts(65_940_000, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3391`
		//  Estimated: `7248`
		// Minimum execution time: 66_701_000 picoseconds.
		Weight::from_parts(69_041_000, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Cancellations` (r:1 w:1)
	/// Proof: `Democracy::Cancellations` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `299`
		//  Estimated: `3666`
		// Minimum execution time: 30_450_000 picoseconds.
		Weight::from_parts(30_970_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(15502), added: 15997, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(2030), added: 4505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:3 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:0 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(2038), added: 4513, mode: `MaxEncodedLen`)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4887`
		//  Estimated: `16987`
		// Minimum execution time: 116_121_000 picoseconds.
		Weight::from_parts(118_121_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(2038), added: 4513, mode: `MaxEncodedLen`)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2161`
		//  Estimated: `5503`
		// Minimum execution time: 16_500_000 picoseconds.
		Weight::from_parts(17_040_000, 0)
			.saturating_add(Weight::from_parts(0, 5503))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_570_000 picoseconds.
		Weight::from_parts(3_730_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_590_000 picoseconds.
		Weight::from_parts(3_800_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:1)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:2)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3518`
		// Minimum execution time: 29_071_000 picoseconds.
		Weight::from_parts(29_880_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(2038), added: 4513, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2264`
		//  Estimated: `5503`
		// Minimum execution time: 33_430_000 picoseconds.
		Weight::from_parts(34_420_000, 0)
			.saturating_add(Weight::from_parts(0, 5503))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(15502), added: 15997, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(2030), added: 4505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4786`
		//  Estimated: `16987`
		// Minimum execution time: 94_101_000 picoseconds.
		Weight::from_parts(95_311_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204`
		//  Estimated: `3518`
		// Minimum execution time: 20_860_000 picoseconds.
		Weight::from_parts(21_140_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + r * (86 ±0)`
		//  Estimated: `1489 + r * (2676 ±0)`
		// Minimum execution time: 7_750_000 picoseconds.
		Weight::from_parts(7_968_697, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 12_219
			.saturating_add(Weight::from_parts(4_763_910, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::LastTabledWasExternal` (r:1 w:0)
	/// Proof: `Democracy::LastTabledWasExternal` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(15502), added: 15997, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `177 + r * (86 ±0)`
		//  Estimated: `16987 + r * (2676 ±0)`
		// Minimum execution time: 11_290_000 picoseconds.
		Weight::from_parts(12_285_804, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			// Standard Error: 11_028
			.saturating_add(Weight::from_parts(4_756_386, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:3 w:3)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `695 + r * (108 ±0)`
		//  Estimated: `19764 + r * (2676 ±0)`
		// Minimum execution time: 55_030_000 picoseconds.
		Weight::from_parts(55_072_861, 0)
			.saturating_add(Weight::from_parts(0, 19764))
			// Standard Error: 11_529
			.saturating_add(Weight::from_parts(5_742_435, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:2 w:2)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388 + r * (108 ±0)`
		//  Estimated: `13506 + r * (2676 ±0)`
		// Minimum execution time: 24_680_000 picoseconds.
		Weight::from_parts(21_598_217, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			// Standard Error: 12_887
			.saturating_add(Weight::from_parts(5_725_695, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::PublicProps` (r:0 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(15502), added: 15997, mode: `MaxEncodedLen`)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_380_000 picoseconds.
		Weight::from_parts(3_560_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `7248`
		// Minimum execution time: 28_350_000 picoseconds.
		Weight::from_parts(40_236_987, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 3_801
			.saturating_add(Weight::from_parts(55_602, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(937), added: 3412, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `457 + r * (22 ±0)`
		//  Estimated: `7248`
		// Minimum execution time: 39_830_000 picoseconds.
		Weight::from_parts(41_643_881, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 920
			.saturating_add(Weight::from_parts(64_555, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `649 + r * (26 ±0)`
		//  Estimated: `7248`
		// Minimum execution time: 21_010_000 picoseconds.
		Weight::from_parts(24_788_736, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 3_331
			.saturating_add(Weight::from_parts(136_570, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3783), added: 6258, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `649 + r * (26 ±0)`
		//  Estimated: `7248`
		// Minimum execution time: 21_130_000 picoseconds.
		Weight::from_parts(24_796_439, 0)
			.saturating_add(Weight::from_parts(0, 7248))
			// Standard Error: 3_679
			.saturating_add(Weight::from_parts(144_997, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `251`
		//  Estimated: `3544`
		// Minimum execution time: 23_360_000 picoseconds.
		Weight::from_parts(23_920_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3518`
		// Minimum execution time: 19_680_000 picoseconds.
		Weight::from_parts(19_980_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(15502), added: 15997, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3583`
		//  Estimated: `16987`
		// Minimum execution time: 43_750_000 picoseconds.
		Weight::from_parts(44_700_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(15502), added: 15997, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3555`
		//  Estimated: `16987`
		// Minimum execution time: 39_170_000 picoseconds.
		Weight::from_parts(40_450_000, 0)
			.saturating_add(Weight::from_parts(0, 16987))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3544`
		// Minimum execution time: 17_450_000 picoseconds.
		Weight::from_parts(17_941_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235`
		//  Estimated: `3666`
		// Minimum execution time: 22_991_000 picoseconds.
		Weight::from_parts(23_480_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
