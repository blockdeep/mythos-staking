
//! Autogenerated weights for `pallet_myth_proxy`
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
// pallet_myth_proxy
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_myth_proxy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_myth_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_myth_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: `MythProxy::SponsorshipApprovals` (r:1 w:1)
	/// Proof: `MythProxy::SponsorshipApprovals` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::SponsorAgents` (r:1 w:0)
	/// Proof: `MythProxy::SponsorAgents` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::Proxies` (r:0 w:1)
	/// Proof: `MythProxy::Proxies` (`max_values`: None, `max_size`: Some(94), added: 2569, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::ApprovalsByAgent` (r:0 w:1)
	/// Proof: `MythProxy::ApprovalsByAgent` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	fn add_proxy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `454`
		//  Estimated: `3610`
		// Minimum execution time: 71_880_000 picoseconds.
		Weight::from_parts(72_550_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `MythProxy::Proxies` (r:1 w:1)
	/// Proof: `MythProxy::Proxies` (`max_values`: None, `max_size`: Some(94), added: 2569, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	fn remove_proxy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3610`
		// Minimum execution time: 51_841_000 picoseconds.
		Weight::from_parts(52_910_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `MythProxy::Proxies` (r:1 w:0)
	/// Proof: `MythProxy::Proxies` (`max_values`: None, `max_size`: Some(94), added: 2569, mode: `MaxEncodedLen`)
	fn proxy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `3559`
		// Minimum execution time: 19_700_000 picoseconds.
		Weight::from_parts(20_220_000, 0)
			.saturating_add(Weight::from_parts(0, 3559))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `MythProxy::SponsorAgents` (r:1 w:0)
	/// Proof: `MythProxy::SponsorAgents` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::SponsorshipApprovals` (r:0 w:1)
	/// Proof: `MythProxy::SponsorshipApprovals` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::ApprovalsByAgent` (r:0 w:1)
	/// Proof: `MythProxy::ApprovalsByAgent` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	fn approve_proxy_funding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227`
		//  Estimated: `3521`
		// Minimum execution time: 18_790_000 picoseconds.
		Weight::from_parts(19_650_000, 0)
			.saturating_add(Weight::from_parts(0, 3521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `MythProxy::SponsorAgents` (r:1 w:1)
	/// Proof: `MythProxy::SponsorAgents` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn register_sponsor_agent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3521`
		// Minimum execution time: 13_970_000 picoseconds.
		Weight::from_parts(14_840_000, 0)
			.saturating_add(Weight::from_parts(0, 3521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `MythProxy::SponsorAgents` (r:1 w:1)
	/// Proof: `MythProxy::SponsorAgents` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::InvalidatedAgents` (r:0 w:1)
	/// Proof: `MythProxy::InvalidatedAgents` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn revoke_sponsor_agent() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227`
		//  Estimated: `3521`
		// Minimum execution time: 17_591_000 picoseconds.
		Weight::from_parts(18_700_000, 0)
			.saturating_add(Weight::from_parts(0, 3521))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `MythProxy::Proxies` (r:1 w:1)
	/// Proof: `MythProxy::Proxies` (`max_values`: None, `max_size`: Some(94), added: 2569, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	fn remove_sponsored_proxy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413`
		//  Estimated: `3610`
		// Minimum execution time: 52_630_000 picoseconds.
		Weight::from_parts(53_110_000, 0)
			.saturating_add(Weight::from_parts(0, 3610))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `MythProxy::InvalidatedAgents` (r:2 w:0)
	/// Proof: `MythProxy::InvalidatedAgents` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::ApprovalsByAgent` (r:2 w:1)
	/// Proof: `MythProxy::ApprovalsByAgent` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `MythProxy::SponsorshipApprovals` (r:1 w:1)
	/// Proof: `MythProxy::SponsorshipApprovals` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn cleanup_approvals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `423`
		//  Estimated: `6124`
		// Minimum execution time: 28_520_000 picoseconds.
		Weight::from_parts(29_550_000, 0)
			.saturating_add(Weight::from_parts(0, 6124))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
