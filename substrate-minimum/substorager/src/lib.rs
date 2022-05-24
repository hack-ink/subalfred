// crates.io
#[cfg(feature = "codec")] use parity_scale_codec::{Decode, Encode};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub enum StorageType {
	Plain(String),
	Map { hashers: Vec<StorageHasher>, key: u64, value: u64 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
	pub fn hash(&self, data: &[u8]) -> Vec<u8> {
		match self {
			StorageHasher::Blake2_128 => subhasher::blake2_128(data).to_vec(),
			StorageHasher::Blake2_256 => subhasher::blake2_256(data).to_vec(),
			StorageHasher::Blake2_128Concat => subhasher::blake2_128_concat(data),
			StorageHasher::Twox128 => subhasher::twox_128(data).to_vec(),
			StorageHasher::Twox256 => subhasher::twox_256(data).to_vec(),
			StorageHasher::Twox64Concat => subhasher::twox_64_concat(data),
			StorageHasher::Identity => subhasher::identity(data).as_ref().to_vec(),
		}
	}
}

pub fn storage_key(prefix: &[u8], item: &[u8]) -> Vec<u8> {
	let mut storage_key = Vec::new();
	storage_key.extend_from_slice(&subhasher::twox_128(prefix));
	storage_key.extend_from_slice(&subhasher::twox_128(item));

	storage_key
}
pub fn hex_storage_key_with_prefix(hex_prefix: &str, prefix: &[u8], item: &[u8]) -> String {
	array_bytes::bytes2hex(hex_prefix, storage_key(prefix, item))
}

pub fn storage_map_key(prefix: &[u8], item: &[u8], key: (&StorageHasher, &[u8])) -> Vec<u8> {
	let mut storage_map_key = storage_key(prefix, item);
	storage_map_key.extend_from_slice(&key.0.hash(key.1));

	storage_map_key
}

pub fn hex_storage_map_key_with_prefix(
	hex_prefix: &str,
	prefix: &[u8],
	item: &[u8],
	key: (&StorageHasher, &[u8]),
) -> String {
	array_bytes::bytes2hex(hex_prefix, storage_map_key(prefix, item, key))
}

pub fn storage_double_map_key(
	prefix: &[u8],
	item: &[u8],
	key1: (StorageHasher, &[u8]),
	key2: (StorageHasher, &[u8]),
) -> Vec<u8> {
	let mut storage_double_map_key = storage_key(prefix, item);
	storage_double_map_key.extend_from_slice(&key1.0.hash(key1.1));
	storage_double_map_key.extend_from_slice(&key2.0.hash(key2.1));

	storage_double_map_key
}
pub fn hex_storage_double_map_key_with_prefix(
	hex_prefix: &str,
	prefix: &[u8],
	item: &[u8],
	key1: (StorageHasher, &[u8]),
	key2: (StorageHasher, &[u8]),
) -> String {
	array_bytes::bytes2hex(hex_prefix, storage_double_map_key(prefix, item, key1, key2))
}
