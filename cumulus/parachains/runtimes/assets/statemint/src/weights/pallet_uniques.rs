// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_uniques`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemint-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_uniques
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemint/src/weights/pallet_uniques.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::WeightInfo for WeightInfo<T> {
	// Storage: Uniques NextCollectionId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn create() -> Weight {
		(27_424_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques NextCollectionId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		(17_098_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Attribute (r:0 w:1000)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: Uniques Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((10_919_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 18_000
			.saturating_add((1_679_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 18_000
			.saturating_add((1_463_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn mint() -> Weight {
		(36_329_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn burn() -> Weight {
		(36_746_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		(28_916_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:100 w:100)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 15_000
			.saturating_add((12_745_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn freeze() -> Weight {
		(21_813_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn thaw() -> Weight {
		(22_088_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		(17_184_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		(17_311_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		(25_174_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn set_team() -> Weight {
		(17_879_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		(20_491_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		(42_444_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		(41_217_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		(34_559_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		(34_863_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		(33_199_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Done by hand to satisfy master branch
	// This will be regenerated for the next release
	fn try_increment_id() -> Weight {
		(20_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		(31_472_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		(23_618_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		(23_507_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		(21_120_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		(20_664_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn set_price() -> Weight {
		(19_938_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques ItemPriceOf (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	fn buy_item() -> Weight {
		(39_254_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
