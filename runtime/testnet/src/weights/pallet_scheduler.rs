
//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.0.0
//! DATE: 2025-03-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `versi-developer-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local-v")`, DB CACHE: 1024

// Executed Command:
// ./target/release/mythos-node
// benchmark
// pallet
// --chain
// local-v
// --pallet
// pallet_scheduler
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_scheduler.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	/// Storage: `Scheduler::IncompleteSince` (r:1 w:1)
	/// Proof: `Scheduler::IncompleteSince` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn service_agendas_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `30`
		//  Estimated: `1489`
		// Minimum execution time: 3_638_000 picoseconds.
		Weight::from_parts(3_896_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80 + s * (177 ±0)`
		//  Estimated: `402327`
		// Minimum execution time: 3_886_000 picoseconds.
		Weight::from_parts(4_049_000, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			// Standard Error: 1_185
			.saturating_add(Weight::from_parts(567_123, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_493_000 picoseconds.
		Weight::from_parts(3_757_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Preimage::PreimageFor` (r:1 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `141 + s * (1 ±0)`
		//  Estimated: `4197809`
		// Minimum execution time: 19_555_000 picoseconds.
		Weight::from_parts(19_846_000, 0)
			.saturating_add(Weight::from_parts(0, 4197809))
			// Standard Error: 580
			.saturating_add(Weight::from_parts(49_412, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Scheduler::Lookup` (r:0 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn service_task_named() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_190_000 picoseconds.
		Weight::from_parts(5_421_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_periodic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_598_000 picoseconds.
		Weight::from_parts(3_746_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn execute_dispatch_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_370_000 picoseconds.
		Weight::from_parts(2_534_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_366_000 picoseconds.
		Weight::from_parts(2_484_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 511]`.
	fn schedule(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80 + s * (177 ±0)`
		//  Estimated: `402327`
		// Minimum execution time: 11_414_000 picoseconds.
		Weight::from_parts(6_437_633, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			// Standard Error: 2_506
			.saturating_add(Weight::from_parts(630_734, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:0 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 512]`.
	fn cancel(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80 + s * (177 ±0)`
		//  Estimated: `402327`
		// Minimum execution time: 17_603_000 picoseconds.
		Weight::from_parts(3_843_632, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			// Standard Error: 2_782
			.saturating_add(Weight::from_parts(920_966, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 511]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `595 + s * (178 ±0)`
		//  Estimated: `402327`
		// Minimum execution time: 15_184_000 picoseconds.
		Weight::from_parts(12_668_737, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			// Standard Error: 2_591
			.saturating_add(Weight::from_parts(650_286, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 512]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `708 + s * (177 ±0)`
		//  Estimated: `402327`
		// Minimum execution time: 20_131_000 picoseconds.
		Weight::from_parts(9_532_756, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			// Standard Error: 2_609
			.saturating_add(Weight::from_parts(931_913, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 512]`.
	fn schedule_retry(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117`
		//  Estimated: `402327`
		// Minimum execution time: 10_589_000 picoseconds.
		Weight::from_parts(13_090_058, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			// Standard Error: 394
			.saturating_add(Weight::from_parts(20_668, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Scheduler::Agenda` (r:1 w:0)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn set_retry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `90704`
		//  Estimated: `402327`
		// Minimum execution time: 196_293_000 picoseconds.
		Weight::from_parts(208_808_000, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Scheduler::Lookup` (r:1 w:0)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:0)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn set_retry_named() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `91746`
		//  Estimated: `402327`
		// Minimum execution time: 210_306_000 picoseconds.
		Weight::from_parts(219_495_000, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Scheduler::Agenda` (r:1 w:0)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn cancel_retry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `90716`
		//  Estimated: `402327`
		// Minimum execution time: 201_599_000 picoseconds.
		Weight::from_parts(208_469_000, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Scheduler::Lookup` (r:1 w:0)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:0)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(398862), added: 401337, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn cancel_retry_named() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `91758`
		//  Estimated: `402327`
		// Minimum execution time: 203_517_000 picoseconds.
		Weight::from_parts(214_726_000, 0)
			.saturating_add(Weight::from_parts(0, 402327))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
