
//! Autogenerated weights for `pallet_proxy`
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
// pallet_proxy
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_proxy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 17_090_000 picoseconds.
		Weight::from_parts(17_944_761, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 547
			.saturating_add(Weight::from_parts(29_765, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `367 + a * (56 ±0) + p * (25 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 46_780_000 picoseconds.
		Weight::from_parts(47_142_275, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 959
			.saturating_add(Weight::from_parts(170_941, 0).saturating_mul(a.into()))
			// Standard Error: 991
			.saturating_add(Weight::from_parts(27_267, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295 + a * (56 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 29_280_000 picoseconds.
		Weight::from_parts(31_353_107, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 3_109
			.saturating_add(Weight::from_parts(167_552, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295 + a * (56 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 29_321_000 picoseconds.
		Weight::from_parts(29_279_228, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_728
			.saturating_add(Weight::from_parts(187_992, 0).saturating_mul(a.into()))
			// Standard Error: 1_785
			.saturating_add(Weight::from_parts(24_746, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311 + a * (56 ±0) + p * (25 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 38_160_000 picoseconds.
		Weight::from_parts(42_234_612, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_726
			.saturating_add(Weight::from_parts(192_088, 0).saturating_mul(a.into()))
			// Standard Error: 1_783
			.saturating_add(Weight::from_parts(32_133, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 28_390_000 picoseconds.
		Weight::from_parts(29_642_736, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_114
			.saturating_add(Weight::from_parts(36_225, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 28_030_000 picoseconds.
		Weight::from_parts(29_370_570, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_050
			.saturating_add(Weight::from_parts(42_897, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 25_450_000 picoseconds.
		Weight::from_parts(26_447_988, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_835
			.saturating_add(Weight::from_parts(42_243, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127`
		//  Estimated: `4310`
		// Minimum execution time: 30_530_000 picoseconds.
		Weight::from_parts(31_659_157, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 537
			.saturating_add(Weight::from_parts(13_704, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 30]`.
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 26_891_000 picoseconds.
		Weight::from_parts(27_793_549, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 591
			.saturating_add(Weight::from_parts(23_402, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(2233), added: 4708, mode: `MaxEncodedLen`)
	fn poke_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `453`
		//  Estimated: `5698`
		// Minimum execution time: 43_833_000 picoseconds.
		Weight::from_parts(44_489_000, 0)
			.saturating_add(Weight::from_parts(0, 5698))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
