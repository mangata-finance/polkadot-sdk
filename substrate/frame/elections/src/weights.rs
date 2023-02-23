// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_elections
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=pallet_elections
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/elections/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_elections.
pub trait WeightInfo {
	fn vote_equal(v: u32, ) -> Weight;
	fn vote_more(v: u32, ) -> Weight;
	fn vote_less(v: u32, ) -> Weight;
	fn remove_voter() -> Weight;
	fn submit_candidacy(c: u32, ) -> Weight;
	fn renounce_candidacy_candidate(c: u32, ) -> Weight;
	fn renounce_candidacy_members() -> Weight;
	fn renounce_candidacy_runners_up() -> Weight;
	fn remove_member_without_replacement() -> Weight;
	fn remove_member_with_replacement() -> Weight;
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight;
	fn pre_solve_election(c: u32, v: u32, e: u32, ) -> Weight;
	fn post_solve_election(c: u32, v: u32, e: u32, ) -> Weight;
}

/// Weights for pallet_elections using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Minimum execution time: 37_500 nanoseconds.
		Weight::from_ref_time(38_575_649)
			// Standard Error: 2_961
			.saturating_add(Weight::from_ref_time(154_225).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Minimum execution time: 48_836 nanoseconds.
		Weight::from_ref_time(49_566_969)
			// Standard Error: 3_258
			.saturating_add(Weight::from_ref_time(153_342).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Minimum execution time: 47_664 nanoseconds.
		Weight::from_ref_time(48_768_157)
			// Standard Error: 3_321
			.saturating_add(Weight::from_ref_time(215_112).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		// Minimum execution time: 47_146 nanoseconds.
		Weight::from_ref_time(47_846_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Minimum execution time: 42_799 nanoseconds.
		Weight::from_ref_time(46_920_164)
			// Standard Error: 780
			.saturating_add(Weight::from_ref_time(81_672).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Elections Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 40_946 nanoseconds.
		Weight::from_ref_time(53_109_738)
			// Standard Error: 1_220
			.saturating_add(Weight::from_ref_time(60_643).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		// Minimum execution time: 53_454 nanoseconds.
		Weight::from_ref_time(53_921_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		// Minimum execution time: 41_275 nanoseconds.
		Weight::from_ref_time(42_444_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000)
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		// Minimum execution time: 62_040 nanoseconds.
		Weight::from_ref_time(62_569_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Elections Voting (r:5001 w:5000)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 298_212_195 nanoseconds.
		Weight::from_ref_time(298_678_889_000)
			// Standard Error: 264_713
			.saturating_add(Weight::from_ref_time(38_222_955).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:10001 w:0)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn pre_solve_election(_c: u32, v: u32, _e: u32, ) -> Weight {
		// Minimum execution time: 6_543_626 nanoseconds.
		Weight::from_ref_time(6_627_885_000)
			// Standard Error: 14_605
			.saturating_add(Weight::from_ref_time(7_613_226).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(296))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Elections ElectionRounds (r:1 w:1)
	// Storage: Elections Candidates (r:0 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn post_solve_election(c: u32, v: u32, _e: u32, ) -> Weight {
		// Minimum execution time: 3_843_566 nanoseconds.
		Weight::from_ref_time(3_854_020_000)
			// Standard Error: 59_766
			.saturating_add(Weight::from_ref_time(9_622_797).saturating_mul(c.into()))
			// Standard Error: 5_975
			.saturating_add(Weight::from_ref_time(541_471).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Minimum execution time: 37_500 nanoseconds.
		Weight::from_ref_time(38_575_649)
			// Standard Error: 2_961
			.saturating_add(Weight::from_ref_time(154_225).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Minimum execution time: 48_836 nanoseconds.
		Weight::from_ref_time(49_566_969)
			// Standard Error: 3_258
			.saturating_add(Weight::from_ref_time(153_342).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Minimum execution time: 47_664 nanoseconds.
		Weight::from_ref_time(48_768_157)
			// Standard Error: 3_321
			.saturating_add(Weight::from_ref_time(215_112).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Elections Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		// Minimum execution time: 47_146 nanoseconds.
		Weight::from_ref_time(47_846_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Elections Candidates (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Minimum execution time: 42_799 nanoseconds.
		Weight::from_ref_time(46_920_164)
			// Standard Error: 780
			.saturating_add(Weight::from_ref_time(81_672).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Elections Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 40_946 nanoseconds.
		Weight::from_ref_time(53_109_738)
			// Standard Error: 1_220
			.saturating_add(Weight::from_ref_time(60_643).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		// Minimum execution time: 53_454 nanoseconds.
		Weight::from_ref_time(53_921_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Elections RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		// Minimum execution time: 41_275 nanoseconds.
		Weight::from_ref_time(42_444_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000)
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		// Minimum execution time: 62_040 nanoseconds.
		Weight::from_ref_time(62_569_000)
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(5))
	}
	// Storage: Elections Voting (r:5001 w:5000)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 298_212_195 nanoseconds.
		Weight::from_ref_time(298_678_889_000)
			// Standard Error: 264_713
			.saturating_add(Weight::from_ref_time(38_222_955).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(v.into())))
	}
	// Storage: Elections Candidates (r:1 w:0)
	// Storage: Elections Members (r:1 w:0)
	// Storage: Elections RunnersUp (r:1 w:0)
	// Storage: Elections Voting (r:10001 w:0)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn pre_solve_election(_c: u32, v: u32, _e: u32, ) -> Weight {
		// Minimum execution time: 6_543_626 nanoseconds.
		Weight::from_ref_time(6_627_885_000)
			// Standard Error: 14_605
			.saturating_add(Weight::from_ref_time(7_613_226).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(296))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(v.into())))
	}
	// Storage: Elections Members (r:1 w:1)
	// Storage: Elections RunnersUp (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Elections ElectionRounds (r:1 w:1)
	// Storage: Elections Candidates (r:0 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn post_solve_election(c: u32, v: u32, _e: u32, ) -> Weight {
		// Minimum execution time: 3_843_566 nanoseconds.
		Weight::from_ref_time(3_854_020_000)
			// Standard Error: 59_766
			.saturating_add(Weight::from_ref_time(9_622_797).saturating_mul(c.into()))
			// Standard Error: 5_975
			.saturating_add(Weight::from_ref_time(541_471).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes(6))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}
