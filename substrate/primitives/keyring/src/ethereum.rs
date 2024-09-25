//   {
//     name: 'Alith',
//     p: '0x02509540919faacf9ab52146c9aa40db68172d83777250b28e4679176e49ccdd9f',
//     s: '0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133', // nosemgrep
//     type: 'ethereum'
//   },
//   {
//     name: 'Baltathar',
//     p: '0x033bc19e36ff1673910575b6727a974a9abd80c9a875d41ab3e2648dbfb9e4b518',
//     s: '0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b', // nosemgrep
//     type: 'ethereum'
//   },
//   {
//     name: 'Charleth',
//     p: '0x0234637bdc0e89b5d46543bcbf8edff329d2702bc995e27e9af4b1ba009a3c2a5e',
//     s: '0x0b6e18cafb6ed99687ec547bd28139cafdd2bffe70e6b688025de6b445aa5c5b', // nosemgrep
//     type: 'ethereum'
//   },
//   {
//     name: 'Dorothy',
//     p: '0x02a00d60b2b408c2a14c5d70cdd2c205db8985ef737a7e55ad20ea32cc9e7c417c',
//     s: '0x39539ab1876910bbf3a223d84a29e28f1cb4e2e456503e7e91ed39b2e7223d68', // nosemgrep
//     type: 'ethereum'
//   },
//   {
//     name: 'Ethan',
//     p: '0x025cdc005b752651cd3f728fb9192182acb3a9c89e19072cbd5b03f3ee1f1b3ffa',
//     s: '0x7dce9bc8babb68fec1409be38c8e1a52650206a7ed90ff956ae8a6d15eeaaef4', // nosemgrep
//     type: 'ethereum'
//   },
//   {
//     name: 'Faith',
//     p: '0x037964b6c9d546da4646ada28a99e34acaa1d14e7aba861a9055f9bd200c8abf74',
//     s: '0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df', // nosemgrep
//     type: 'ethereum'
//   }

// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Support code for the runtime. A set of test accounts.

pub use sp_core::ecdsa;
use sp_core::{
	ecdsa::{Pair, Public, Signature},
	hex2array, ByteArray, Pair as PairT,
};
use sp_runtime::AccountId20;

extern crate alloc;
use alloc::{fmt, format, str::FromStr, string::String, vec::Vec};

/// Set of test accounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Keyring {
	Alith,
	Baltathar,
	Charleth,
	Dorothy,
	Ethan,
	Faith,
}

impl Keyring {
	pub fn from_public(who: &Public) -> Option<Keyring> {
		Self::iter().find(|&k| &Public::from(k) == who)
	}

	pub fn from_account_id(who: &AccountId20) -> Option<Keyring> {
		Self::iter().find(|&k| &k.to_account_id() == who)
	}

	pub fn from_raw_public(who: [u8; 33]) -> Option<Keyring> {
		Self::from_public(&Public::from_raw(who))
	}

	pub fn to_raw_public(self) -> [u8; 33] {
		*Public::from(self).as_array_ref()
	}

	pub fn to_raw_public_vec(self) -> Vec<u8> {
		Public::from(self).to_raw_vec()
	}

	pub fn to_account_id(self) -> AccountId20 {
		Public::from(self).into()
	}

	#[cfg(feature = "std")]
	pub fn sign(self, msg: &[u8]) -> Signature {
		Pair::from(self).sign(msg)
	}

	pub fn pair(self) -> Pair {
		// Naked unwrap here!!
		let seed = seed(self);
		Pair::from_seed_slice(&seed).expect("static values are known good; qed")
	}

	/// Returns an iterator over all test accounts.
	pub fn iter() -> impl Iterator<Item = Keyring> {
		<Self as strum::IntoEnumIterator>::iter()
	}

	pub fn public(self) -> Public {
		Public::from(self)
	}

	pub fn to_seed(self) -> Vec<u8> {
		seed(self).to_vec()
	}
}

impl From<Keyring> for &'static str {
	fn from(k: Keyring) -> Self {
		match k {
			Keyring::Alith => "Alith",
			Keyring::Baltathar => "Baltathar",
			Keyring::Charleth => "Charleth",
			Keyring::Dorothy => "Dorothy",
			Keyring::Ethan => "Ethan",
			Keyring::Faith => "Faith",
		}
	}
}

impl From<Keyring> for sp_runtime::account::EthereumSigner {
	fn from(x: Keyring) -> Self {
		Public::from(x).into()
	}
}

#[derive(Debug)]
pub struct ParseKeyringError;

impl fmt::Display for ParseKeyringError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ParseKeyringError")
	}
}

impl FromStr for Keyring {
	type Err = ParseKeyringError;

	fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
		match s {
			"Alith" => Ok(Keyring::Alith),
			"Baltathar" => Ok(Keyring::Baltathar),
			"Charleth" => Ok(Keyring::Charleth),
			"Dorothy" => Ok(Keyring::Dorothy),
			"Ethan" => Ok(Keyring::Ethan),
			"Faith" => Ok(Keyring::Faith),
			_ => Err(ParseKeyringError),
		}
	}
}

fn seed(keyring: Keyring) -> [u8; 32] {
	match keyring {
		Keyring::Alith =>
			hex2array!("5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"),
		Keyring::Baltathar =>
			hex2array!("8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b"),
		Keyring::Charleth =>
			hex2array!("0b6e18cafb6ed99687ec547bd28139cafdd2bffe70e6b688025de6b445aa5c5b"),
		Keyring::Dorothy =>
			hex2array!("39539ab1876910bbf3a223d84a29e28f1cb4e2e456503e7e91ed39b2e7223d68"),
		Keyring::Ethan =>
			hex2array!("7dce9bc8babb68fec1409be38c8e1a52650206a7ed90ff956ae8a6d15eeaaef4"),
		Keyring::Faith =>
			hex2array!("b9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df"),
	}
}

impl From<Keyring> for AccountId20 {
	fn from(k: Keyring) -> Self {
		k.to_account_id()
	}
}

impl From<Keyring> for Public {
	fn from(k: Keyring) -> Self {
		Public::from_raw(k.into())
	}
}

impl From<Keyring> for Pair {
	fn from(k: Keyring) -> Self {
		k.pair()
	}
}

impl From<Keyring> for [u8; 33] {
	fn from(k: Keyring) -> Self {
		match k {
			Keyring::Alith =>
				hex2array!("02509540919faacf9ab52146c9aa40db68172d83777250b28e4679176e49ccdd9f"),
			Keyring::Baltathar =>
				hex2array!("033bc19e36ff1673910575b6727a974a9abd80c9a875d41ab3e2648dbfb9e4b518"),
			Keyring::Charleth =>
				hex2array!("0234637bdc0e89b5d46543bcbf8edff329d2702bc995e27e9af4b1ba009a3c2a5e"),
			Keyring::Dorothy =>
				hex2array!("02a00d60b2b408c2a14c5d70cdd2c205db8985ef737a7e55ad20ea32cc9e7c417c"),
			Keyring::Ethan =>
				hex2array!("025cdc005b752651cd3f728fb9192182acb3a9c89e19072cbd5b03f3ee1f1b3ffa"),
			Keyring::Faith =>
				hex2array!("037964b6c9d546da4646ada28a99e34acaa1d14e7aba861a9055f9bd200c8abf74"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::{ecdsa::Pair, Pair as PairT};

	#[test]
	fn should_work() {
		assert!(Pair::verify(
			&Keyring::Alith.sign(b"I am Alith!"),
			b"I am Alith!",
			&Keyring::Alith.public(),
		));
		assert!(!Pair::verify(
			&Keyring::Alith.sign(b"I am Alith!"),
			b"I am Baltathar!",
			&Keyring::Alith.public(),
		));
		assert!(!Pair::verify(
			&Keyring::Alith.sign(b"I am Alith!"),
			b"I am Alith!",
			&Keyring::Baltathar.public(),
		));
	}
	#[test]
	fn verify_static_public_keys() {
		assert!(Keyring::iter().all(|k| { k.pair().public().as_ref() == <[u8; 33]>::from(k) }));
	}
}
