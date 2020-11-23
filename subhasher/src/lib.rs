// --- std ---
use std::hash::Hasher;
// --- crates.io ---
use blake2_rfc::blake2b::blake2b;
use byteorder::{ByteOrder, LittleEndian};
use twox_hash::XxHash;

pub fn blake2_128(data: impl AsRef<[u8]>) -> [u8; 16] {
	let mut dest = [0; 16];
	dest.copy_from_slice(blake2b(16, &[], data.as_ref()).as_bytes());

	dest
}

pub fn blake2_256(data: impl AsRef<[u8]>) -> [u8; 32] {
	let mut dest = [0; 32];
	dest.copy_from_slice(blake2b(32, &[], data.as_ref()).as_bytes());

	dest
}

pub fn blake2_128_concat(data: impl AsRef<[u8]>) -> Vec<u8> {
	let data = data.as_ref();
	let mut v = blake2_128(data).to_vec();
	v.extend_from_slice(data);

	v
}

fn twox(dest: &mut [u8], data: impl AsRef<[u8]>, seed: u64) {
	let mut h = XxHash::with_seed(seed);
	let i = seed as usize * 8;

	h.write(data.as_ref());
	LittleEndian::write_u64(&mut dest[i..i + 8], h.finish());
}

pub fn twox_64(data: impl AsRef<[u8]>) -> [u8; 8] {
	let mut dest = [0; 8];
	twox(&mut dest, data, 0);

	dest
}

pub fn twox_128(data: impl AsRef<[u8]>) -> [u8; 16] {
	let mut dest = [0; 16];
	twox(&mut dest, &data, 0);
	twox(&mut dest, &data, 1);

	dest
}

pub fn twox_256(data: impl AsRef<[u8]>) -> [u8; 32] {
	let mut dest = [0; 32];
	twox(&mut dest, &data, 0);
	twox(&mut dest, &data, 1);
	twox(&mut dest, &data, 2);
	twox(&mut dest, &data, 3);

	dest
}

pub fn twox_64_concat(data: impl AsRef<[u8]>) -> Vec<u8> {
	let data = data.as_ref();
	let mut v = twox_64(data).to_vec();
	v.extend_from_slice(data);

	v
}

pub fn identity(data: impl AsRef<[u8]>) -> impl AsRef<[u8]> {
	data
}
