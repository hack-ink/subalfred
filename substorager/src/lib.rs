// --- std ---
// use std::str::FromStr;

#[derive(Debug)]
pub enum StorageType {
	Plain,
	Map(StorageHasher),
	DoubleMap(StorageHasher, StorageHasher),
}

#[derive(Debug)]
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
			Blake2_128 => blake2_128(data).to_vec(),
			Blake2_256 => blake2_256(data).to_vec(),
			Blake2_128Concat => blake2_128_concat(data).to_vec(),
			Twox128 => twox_128(data).to_vec(),
			Twox256 => twox_256(data).to_vec(),
			Twox64Concat => twox_64_concat(data).to_vec(),
			Identity => identity(data).as_ref().to_vec(),
		}
	}
}
// impl FromStr for StorageHasher {
// 	type Err = ();

// 	fn from_str(s: &str) -> Result<Self, Self::Err> {
// 		// --- substorager ---
// 		use StorageHasher::*;

// 		match s {}
// 	}
// }

pub fn storage_value_key(module: impl AsRef<[u8]>, item: impl AsRef<[u8]>) -> Vec<u8> {
	let mut storage_value_key = vec![];
	storage_value_key.extend_from_slice(&twox_128(module));
	storage_value_key.extend_from_slice(&twox_128(item));

	storage_value_key
}

pub fn storage_map_key(
	module: impl AsRef<[u8]>,
	item: impl AsRef<[u8]>,
	key: (StorageHasher, impl AsRef<[u8]>),
) -> Vec<u8> {
	let mut storage_map_key = storage_value_key(module, item);
	storage_map_key.extend_from_slice(&key.0.hash(key.1));

	storage_map_key
}

pub fn storage_double_map_key(
	module: impl AsRef<[u8]>,
	item: impl AsRef<[u8]>,
	key1: (StorageHasher, impl AsRef<[u8]>),
	key2: (StorageHasher, impl AsRef<[u8]>),
) -> Vec<u8> {
	let mut storage_double_map_key = storage_value_key(module, item);
	storage_double_map_key.extend_from_slice(&key1.0.hash(key1.1));
	storage_double_map_key.extend_from_slice(&key2.0.hash(key2.1));

	storage_double_map_key
}
