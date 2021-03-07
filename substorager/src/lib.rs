// --- crates.io ---
#[cfg(feature = "codec")]
use parity_scale_codec::Decode;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub enum StorageType {
	Plain(String),
	Map {
		hasher: StorageHasher,
		key: String,
		value: String,
		unused: bool,
	},
	DoubleMap {
		hasher: StorageHasher,
		key1: String,
		key2: String,
		value: String,
		key2_hasher: StorageHasher,
	},
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
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
	pub fn hash(&self, data: impl AsRef<[u8]>) -> Vec<u8> {
		// --- substorager ---
		use StorageHasher::*;

		match self {
			Blake2_128 => subhasher::blake2_128(data).to_vec(),
			Blake2_256 => subhasher::blake2_256(data).to_vec(),
			Blake2_128Concat => subhasher::blake2_128_concat(data).to_vec(),
			Twox128 => subhasher::twox_128(data).to_vec(),
			Twox256 => subhasher::twox_256(data).to_vec(),
			Twox64Concat => subhasher::twox_64_concat(data).to_vec(),
			Identity => subhasher::identity(data).as_ref().to_vec(),
		}
	}
}
impl AsRef<StorageHasher> for StorageHasher {
	fn as_ref(&self) -> &Self {
		&self
	}
}

pub fn storage_key(prefix: impl AsRef<[u8]>, item: impl AsRef<[u8]>) -> Vec<u8> {
	let mut storage_key = vec![];
	storage_key.extend_from_slice(&subhasher::twox_128(prefix));
	storage_key.extend_from_slice(&subhasher::twox_128(item));

	storage_key
}
pub fn hex_storage_key_with_prefix(
	hex_prefix: impl AsRef<str>,
	prefix: impl AsRef<[u8]>,
	item: impl AsRef<[u8]>,
) -> String {
	array_bytes::bytes2hex(hex_prefix, storage_key(prefix, item))
}

pub fn storage_map_key(
	prefix: impl AsRef<[u8]>,
	item: impl AsRef<[u8]>,
	key: (impl AsRef<StorageHasher>, impl AsRef<[u8]>),
) -> Vec<u8> {
	let mut storage_map_key = storage_key(prefix, item);
	storage_map_key.extend_from_slice(&key.0.as_ref().hash(key.1));

	storage_map_key
}
pub fn hex_storage_map_key_with_prefix(
	hex_prefix: impl AsRef<str>,
	prefix: impl AsRef<[u8]>,
	item: impl AsRef<[u8]>,
	key: (impl AsRef<StorageHasher>, impl AsRef<[u8]>),
) -> String {
	array_bytes::bytes2hex(hex_prefix, storage_map_key(prefix, item, key))
}

pub fn storage_double_map_key(
	prefix: impl AsRef<[u8]>,
	item: impl AsRef<[u8]>,
	key1: (StorageHasher, impl AsRef<[u8]>),
	key2: (StorageHasher, impl AsRef<[u8]>),
) -> Vec<u8> {
	let mut storage_double_map_key = storage_key(prefix, item);
	storage_double_map_key.extend_from_slice(&key1.0.hash(key1.1));
	storage_double_map_key.extend_from_slice(&key2.0.hash(key2.1));

	storage_double_map_key
}
pub fn hex_storage_double_map_key_with_prefix(
	hex_prefix: impl AsRef<str>,
	prefix: impl AsRef<[u8]>,
	item: impl AsRef<[u8]>,
	key1: (StorageHasher, impl AsRef<[u8]>),
	key2: (StorageHasher, impl AsRef<[u8]>),
) -> String {
	array_bytes::bytes2hex(hex_prefix, storage_double_map_key(prefix, item, key1, key2))
}
