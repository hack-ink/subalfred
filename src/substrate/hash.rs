// --- std ---
use std::hash::Hasher;
// --- crates.io ---
use blake2_rfc::blake2b::blake2b;
use byteorder::{ByteOrder, LittleEndian};
use twox_hash::XxHash;
// --- subalfred ---
use crate::util::{bytes, hex};

pub fn hash(data: &str, hasher: &str, is_hex: bool) -> String {
	let (data, bytes) = if is_hex {
		let data = data.trim_start_matches("0x");

		(format!(r#"hex("0x{}")"#, data), bytes(data))
	} else {
		(format!(r#""{}""#, data), data.as_bytes().to_vec())
	};

	match hasher {
		"blake2-128" => format!("blake2_128({}) = {}", data, hex(&blake2_128(&bytes))),
		"blake2-256" => format!("blake2_256({}) = {}", data, hex(&blake2_256(&bytes))),
		"blake2-128-concat" => {
			format!("blake2_128_concat({}) = {}", data, hex(&blake2_128_concat(&bytes)))
		}
		"twox-64" => format!("twox_64({}) = {}", data, hex(&twox_64(&bytes))),
		"twox-128" => format!("twox_128({}) = {}", data, hex(&twox_128(&bytes))),
		"twox-256" => format!("twox256({}) = {}", data, hex(&twox_256(&bytes))),
		"twox-128-concat" => format!("twox_128_concat({}) = {}", data, hex(&twox_64_concat(&bytes))),
		"identity" => format!("identity({}) = {}", data, hex(&bytes)),
		_ => unreachable!(),
	}
}

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

pub fn blake2_128(data: &[u8]) -> [u8; 16] {
	let mut dest = [0; 16];
	dest.copy_from_slice(blake2b(16, &[], data).as_bytes());

	dest
}

pub fn blake2_256(data: &[u8]) -> [u8; 32] {
	let mut dest = [0; 32];
	dest.copy_from_slice(blake2b(32, &[], data).as_bytes());

	dest
}

pub fn blake2_128_concat(data: &[u8]) -> Vec<u8> {
	let mut v = blake2_128(data).to_vec();
	v.extend_from_slice(data);

	v
}

fn twox(dest: &mut [u8], data: &[u8], seed: u64) {
	let mut h = XxHash::with_seed(seed);
	let i = seed as usize * 8;

	h.write(data);
	LittleEndian::write_u64(&mut dest[i..i + 8], h.finish());
}

pub fn twox_64(data: &[u8]) -> [u8; 8] {
	let mut dest = [0; 8];
	twox(&mut dest, data, 0);

	dest
}

pub fn twox_128(data: &[u8]) -> [u8; 16] {
	let mut dest = [0; 16];
	twox(&mut dest, data, 0);
	twox(&mut dest, data, 1);

	dest
}

pub fn twox_256(data: &[u8]) -> [u8; 32] {
	let mut dest = [0; 32];
	twox(&mut dest, data, 0);
	twox(&mut dest, data, 1);
	twox(&mut dest, data, 2);
	twox(&mut dest, data, 3);

	dest
}

pub fn twox_64_concat(data: &[u8]) -> Vec<u8> {
	let mut v = twox_64(data).to_vec();
	v.extend_from_slice(data);

	v
}
