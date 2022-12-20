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
//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `10`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `6267FC`, CPU: `AMD Ryzen 5 PRO 3600 6-Core Processor`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=10
// --repeat=1
// --pallet=pallet_tips
// --extrinsic=*
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4954`
		// Minimum execution time: 17_173 nanoseconds.
		Weight::from_parts(19_566_311, 4954)
			// Standard Error: 168
			.saturating_add(Weight::from_ref_time(935).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None)
	fn retract_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `2728`
		// Minimum execution time: 15_960 nanoseconds.
		Weight::from_parts(15_960_000, 2728)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None)
	/// Storage: Tips Reasons (r:1 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None)
	/// Storage: Tips Tips (r:0 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 19]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140 + t * (64 ±0)`
		//  Estimated: `3110 + t * (64 ±0)`
		// Minimum execution time: 15_369 nanoseconds.
		Weight::from_parts(16_999_250, 3110)
			// Standard Error: 383
			.saturating_add(Weight::from_ref_time(1_942).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_proof_size(64).saturating_mul(t.into()))
	}
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None)
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None)
	/// The range of component `t` is `[1, 19]`.
	fn tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `393 + t * (112 ±0)`
		//  Estimated: `3363 + t * (112 ±0)`
		// Minimum execution time: 12_324 nanoseconds.
		Weight::from_parts(17_706_524, 3363)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(112).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None)
	/// The range of component `t` is `[1, 19]`.
	fn close_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445 + t * (112 ±0)`
		//  Estimated: `5890 + t * (112 ±0)`
		// Minimum execution time: 27_852 nanoseconds.
		Weight::from_parts(28_873_160, 5890)
			// Standard Error: 31_430
			.saturating_add(Weight::from_ref_time(28_693).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(112).saturating_mul(t.into()))
	}
	/// Storage: Tips Tips (r:1 w:1)
	/// Proof Skipped: Tips Tips (max_values: None, max_size: None)
	/// Storage: Tips Reasons (r:0 w:1)
	/// Proof Skipped: Tips Reasons (max_values: None, max_size: None)
	/// The range of component `t` is `[1, 19]`.
	fn slash_tip(_t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `301`
		//  Estimated: `2776`
		// Minimum execution time: 10_500 nanoseconds.
		Weight::from_parts(16_845_736, 2776)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
