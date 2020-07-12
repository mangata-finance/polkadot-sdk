// This file is part of Substrate.

// Copyright (C) 2019-2020 Parity Technologies (UK) Ltd.
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

//! Test utilities

use super::*;

use std::cell::RefCell;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight, ord_parameter_types};
use sp_core::H256;
use sp_runtime::{
	Perbill, traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system::EnsureSignedBy;

impl_outer_origin! {
	pub enum Origin for Test  where system = frame_system {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const CandidateDeposit: u64 = 25;
	pub const Period: u64 = 4;

	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();

	pub const ExistentialDeposit: u64 = 1;
}
ord_parameter_types! {
	pub const KickOrigin: u64 = 2;
	pub const ScoreOrigin: u64 = 3;
}

impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = ();
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl pallet_balances::Trait for Test {
	type Balance = u64;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

thread_local! {
	pub static MEMBERS: RefCell<Vec<u64>> = RefCell::new(vec![]);
}

pub struct TestChangeMembers;
impl ChangeMembers<u64> for TestChangeMembers {
	fn change_members_sorted(incoming: &[u64], outgoing: &[u64], new: &[u64]) {
		let mut old_plus_incoming = MEMBERS.with(|m| m.borrow().to_vec());
		old_plus_incoming.extend_from_slice(incoming);
		old_plus_incoming.sort();

		let mut new_plus_outgoing = new.to_vec();
		new_plus_outgoing.extend_from_slice(outgoing);
		new_plus_outgoing.sort();

		assert_eq!(old_plus_incoming, new_plus_outgoing);

		MEMBERS.with(|m| *m.borrow_mut() = new.to_vec());
	}
}

impl InitializeMembers<u64> for TestChangeMembers {
	fn initialize_members(new_members: &[u64]) {
		MEMBERS.with(|m| *m.borrow_mut() = new_members.to_vec());
	}
}

impl Trait for Test {
	type Event = ();
	type KickOrigin = EnsureSignedBy<KickOrigin, u64>;
	type MembershipInitialized = TestChangeMembers;
	type MembershipChanged = TestChangeMembers;
	type Currency = Balances;
	type CandidateDeposit = CandidateDeposit;
	type Period = Period;
	type Score = u64;
	type ScoreOrigin = EnsureSignedBy<ScoreOrigin, u64>;
}

type System = frame_system::Module<Test>;
type Balances = pallet_balances::Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(5, 500_000),
			(10, 500_000),
			(15, 500_000),
			(20, 500_000),
			(31, 500_000),
			(40, 500_000),
			(99, 1),
		],
	}.assimilate_storage(&mut t).unwrap();
	GenesisConfig::<Test>{
		pool: vec![
			(5, None),
			(10, Some(1)),
			(20, Some(2)),
			(31, Some(2)),
			(40, Some(3)),
		],
		member_count: 2,
		.. Default::default()
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

/// Fetch an entity from the pool, if existent.
pub fn fetch_from_pool(who: u64) -> Option<(u64, Option<u64>)> {
	<Module<Test>>::pool()
		.into_iter()
		.find(|item| item.0 == who)
}

/// Find an entity in the pool.
/// Returns its position in the `Pool` vec, if existent.
pub fn find_in_pool(who: u64) -> Option<usize> {
	<Module<Test>>::pool()
		.into_iter()
		.position(|item| item.0 == who)
}
