// std
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	ops::Deref,
};
// crates.io
#[cfg(feature = "codec")] use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Default)]
pub struct StorageKey(pub Vec<u8>);
impl StorageKey {
	pub fn new() -> Self {
		Default::default()
	}
}
impl Deref for StorageKey {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl Display for StorageKey {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", array_bytes::bytes2hex("0x", &self.0))
	}
}
impl From<Vec<u8>> for StorageKey {
	fn from(v: Vec<u8>) -> Self {
		Self(v)
	}
}
impl<const N: usize> From<[u8; N]> for StorageKey {
	fn from(a: [u8; N]) -> Self {
		Self(a.to_vec())
	}
}
impl From<&[u8]> for StorageKey {
	fn from(a: &[u8]) -> Self {
		Self(a.to_vec())
	}
}
// impl<S> TryFrom<S> for StorageKey
// where
// 	S: AsRef<str>,
// {
// 	type Error;
//
// 	fn try_from(value: S) -> Result<Self, Self::Error> {
// 		todo!()
// 	}
// }

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub enum StorageHasher {
	Blake2_128,
	Blake2_256,
	Blake2_128Concat,
	Twox128,
	Twox256,
	Twox64Concat,
	Identity,
}
impl StorageHasher {
	pub fn hash(&self, data: &[u8]) -> StorageKey {
		match self {
			StorageHasher::Blake2_128 => subhasher::blake2_128(data).into(),
			StorageHasher::Blake2_256 => subhasher::blake2_256(data).into(),
			StorageHasher::Blake2_128Concat => subhasher::blake2_128_concat(data).into(),
			StorageHasher::Twox128 => subhasher::twox128(data).into(),
			StorageHasher::Twox256 => subhasher::twox256(data).into(),
			StorageHasher::Twox64Concat => subhasher::twox64_concat(data).into(),
			StorageHasher::Identity => subhasher::identity(data).into(),
		}
	}
}

pub fn storage_key(prefix: &[u8], item: &[u8]) -> StorageKey {
	let mut storage_key = Vec::new();

	storage_key.extend_from_slice(&subhasher::twox128(prefix));
	storage_key.extend_from_slice(&subhasher::twox128(item));

	storage_key.into()
}

pub fn storage_map_key(prefix: &[u8], item: &[u8], key: (&StorageHasher, &[u8])) -> StorageKey {
	let mut storage_map_key = storage_key(prefix, item);

	storage_map_key.0.extend_from_slice(&key.0.hash(key.1));

	storage_map_key
}

pub fn storage_double_map_key(
	prefix: &[u8],
	item: &[u8],
	key1: (StorageHasher, &[u8]),
	key2: (StorageHasher, &[u8]),
) -> StorageKey {
	let mut storage_double_map_key = storage_key(prefix, item);

	storage_double_map_key.0.extend_from_slice(&key1.0.hash(key1.1));
	storage_double_map_key.0.extend_from_slice(&key2.0.hash(key2.1));

	storage_double_map_key
}
