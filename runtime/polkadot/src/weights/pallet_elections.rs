// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_elections`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/polkadot/.git/.artifacts/bench.json
// --pallet=pallet_elections
// --chain=polkadot-dev
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_elections`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections::WeightInfo for WeightInfo<T> {
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `369 + v * (80 ±0)`
		//  Estimated: `17670 + v * (320 ±0)`
		// Minimum execution time: 31_083_000 picoseconds.
		Weight::from_parts(32_337_960, 0)
			.saturating_add(Weight::from_parts(0, 17670))
			// Standard Error: 4_880
			.saturating_add(Weight::from_parts(147_172, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `337 + v * (80 ±0)`
		//  Estimated: `17542 + v * (320 ±0)`
		// Minimum execution time: 43_152_000 picoseconds.
		Weight::from_parts(44_392_141, 0)
			.saturating_add(Weight::from_parts(0, 17542))
			// Standard Error: 6_659
			.saturating_add(Weight::from_parts(102_623, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `369 + v * (80 ±0)`
		//  Estimated: `17670 + v * (320 ±0)`
		// Minimum execution time: 42_846_000 picoseconds.
		Weight::from_parts(44_138_547, 0)
			.saturating_add(Weight::from_parts(0, 17670))
			// Standard Error: 5_599
			.saturating_add(Weight::from_parts(137_337, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `891`
		//  Estimated: `12634`
		// Minimum execution time: 41_301_000 picoseconds.
		Weight::from_parts(45_448_000, 0)
			.saturating_add(Weight::from_parts(0, 12634))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2358 + c * (48 ±0)`
		//  Estimated: `11523 + c * (144 ±0)`
		// Minimum execution time: 33_161_000 picoseconds.
		Weight::from_parts(25_421_117, 0)
			.saturating_add(Weight::from_parts(0, 11523))
			// Standard Error: 955
			.saturating_add(Weight::from_parts(87_141, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 144).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `250 + c * (48 ±0)`
		//  Estimated: `1722 + c * (48 ±0)`
		// Minimum execution time: 29_244_000 picoseconds.
		Weight::from_parts(21_012_035, 0)
			.saturating_add(Weight::from_parts(0, 1722))
			// Standard Error: 997
			.saturating_add(Weight::from_parts(63_434, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2599`
		//  Estimated: `18935`
		// Minimum execution time: 45_482_000 picoseconds.
		Weight::from_parts(46_446_000, 0)
			.saturating_add(Weight::from_parts(0, 18935))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1711`
		//  Estimated: `3196`
		// Minimum execution time: 30_072_000 picoseconds.
		Weight::from_parts(30_749_000, 0)
			.saturating_add(Weight::from_parts(0, 3196))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000_000_000 picoseconds.
		Weight::from_parts(2_000_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2599`
		//  Estimated: `22528`
		// Minimum execution time: 62_095_000 picoseconds.
		Weight::from_parts(62_831_000, 0)
			.saturating_add(Weight::from_parts(0, 22528))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: PhragmenElection Voting (r:10001 w:10000)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:10000 w:10000)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:10000 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:10000 w:10000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36028 + v * (808 ±0)`
		//  Estimated: `156102 + v * (14608 ±0)`
		// Minimum execution time: 358_390_607_000 picoseconds.
		Weight::from_parts(360_218_670_000, 0)
			.saturating_add(Weight::from_parts(0, 156102))
			// Standard Error: 302_192
			.saturating_add(Weight::from_parts(43_783_152, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 14608).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:10001 w:0)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn pre_solve_election(_c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + v * (592 ±0) + e * (28 ±0)`
		//  Estimated: `3129520 + v * (3893 ±4) + e * (62 ±0)`
		// Minimum execution time: 3_789_611_000 picoseconds.
		Weight::from_parts(3_833_347_000, 0)
			.saturating_add(Weight::from_parts(0, 3129520))
			// Standard Error: 12_775
			.saturating_add(Weight::from_parts(4_259_276, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(296))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 3893).saturating_mul(v.into()))
			.saturating_add(Weight::from_parts(0, 62).saturating_mul(e.into()))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:967 w:967)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection ElectionRounds (r:1 w:1)
	/// Proof Skipped: PhragmenElection ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Candidates (r:0 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn post_solve_election(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (254 ±0) + v * (16 ±0)`
		//  Estimated: `32696 + c * (3988 ±1) + v * (77 ±0)`
		// Minimum execution time: 712_443_000 picoseconds.
		Weight::from_parts(722_719_000, 0)
			.saturating_add(Weight::from_parts(0, 32696))
			// Standard Error: 44_600
			.saturating_add(Weight::from_parts(26_159_277, 0).saturating_mul(c.into()))
			// Standard Error: 4_459
			.saturating_add(Weight::from_parts(277_261, 0).saturating_mul(v.into()))
			// Standard Error: 286
			.saturating_add(Weight::from_parts(5_031, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 3988).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 77).saturating_mul(v.into()))
	}
}