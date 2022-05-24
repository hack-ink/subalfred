#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))] extern crate alloc;

// core
use core::hash::Hasher as _;
// alloc
#[cfg(not(feature = "std"))] use alloc::vec::Vec;
// crates.io
use blake2_rfc::blake2b::blake2b;
use byteorder::{ByteOrder, LittleEndian};
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher as _, Keccak};
use twox_hash::XxHash;

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

pub fn blake2_512(data: &[u8]) -> [u8; 64] {
	let mut dest = [0; 64];
	dest.copy_from_slice(blake2b(64, &[], data).as_bytes());

	dest
}

pub fn blake2_128_concat(data: &[u8]) -> Vec<u8> {
	let data = data;
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
	twox(&mut dest, &data, 0);
	twox(&mut dest, &data, 1);

	dest
}

pub fn twox_256(data: &[u8]) -> [u8; 32] {
	let mut dest = [0; 32];
	twox(&mut dest, &data, 0);
	twox(&mut dest, &data, 1);
	twox(&mut dest, &data, 2);
	twox(&mut dest, &data, 3);

	dest
}

pub fn twox_64_concat(data: &[u8]) -> Vec<u8> {
	let data = data;
	let mut v = twox_64(data).to_vec();
	v.extend_from_slice(data);

	v
}

pub fn identity<T>(data: T) -> T
where
	T: AsRef<[u8]>,
{
	data
}

pub fn keccak_256(data: &[u8]) -> [u8; 32] {
	let mut keccak = Keccak::v256();
	keccak.update(data);

	let mut output = [0u8; 32];
	keccak.finalize(&mut output);

	output
}

pub fn keccak_512(data: &[u8]) -> [u8; 64] {
	let mut keccak = Keccak::v512();
	keccak.update(data);

	let mut output = [0u8; 64];
	keccak.finalize(&mut output);

	output
}

pub fn sha2_256(data: &[u8]) -> [u8; 32] {
	let mut hasher = Sha256::new();
	hasher.update(data);

	let mut output = [0u8; 32];
	output.copy_from_slice(&hasher.finalize());

	output
}
