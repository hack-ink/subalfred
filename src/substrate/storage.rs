// --- std ---
use std::hash::Hasher;
// --- crates.io ---
use byteorder::{ByteOrder, LittleEndian};
use twox_hash::XxHash;
// --- subalfred ---
use crate::util::hex;

pub fn parse_storage_keys(module: Option<&str>, item: Option<&str>) -> String {
	let mut storage_prefix = String::from("0x");

	if let Some(module) = module {
		storage_prefix.push_str(&hex(&twox_128(module.as_bytes())));
	}
	if let Some(item) = item {
		storage_prefix.push_str(&hex(&twox_128(item.as_bytes())));
	}

	storage_prefix
}

pub fn twox_128(data: &[u8]) -> [u8; 16] {
	let mut dest: [u8; 16] = [0; 16];
	let r0 = {
		let mut h0 = XxHash::with_seed(0);
		h0.write(data);

		h0.finish()
	};
	let r1 = {
		let mut h1 = XxHash::with_seed(1);
		h1.write(data);

		h1.finish()
	};

	LittleEndian::write_u64(&mut dest[0..8], r0);
	LittleEndian::write_u64(&mut dest[8..16], r1);

	dest
}
