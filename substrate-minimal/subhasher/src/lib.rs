//! Minimal implementation of Substrate hash.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

#[cfg(not(feature = "std"))] extern crate alloc;

#[cfg(test)] mod test;

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

/// Hash the data into a 16-bytes array with BLAKE2 algorithm.
pub fn blake2_128<D>(data: D) -> [u8; 16]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
	let mut dest = [0; 16];

	dest.copy_from_slice(blake2b(16, &[], data).as_bytes());

	dest
}

/// Hash the data into a 32-bytes array with BLAKE2 algorithm.
pub fn blake2_256<D>(data: D) -> [u8; 32]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
	let mut dest = [0; 32];

	dest.copy_from_slice(blake2b(32, &[], data).as_bytes());

	dest
}

/// Hash the data into a 64-bytes array with BLAKE2 algorithm.
pub fn blake2_512<D>(data: D) -> [u8; 64]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
	let mut dest = [0; 64];

	dest.copy_from_slice(blake2b(64, &[], data).as_bytes());

	dest
}

/// Hash the data into `blake2_128(data) + data`.
pub fn blake2_128_concat<D>(data: D) -> Vec<u8>
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
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

/// Hash the data into a 8-bytes array with XX algorithm.
pub fn twox64<D>(data: D) -> [u8; 8]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
	let mut dest = [0; 8];

	twox(&mut dest, data, 0);

	dest
}

/// Hash the data into a 16-bytes array with XX algorithm.
pub fn twox128<D>(data: D) -> [u8; 16]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
	let mut dest = [0; 16];

	twox(&mut dest, data, 0);
	twox(&mut dest, data, 1);

	dest
}

/// Hash the data into a 32-bytes array with XX algorithm.
pub fn twox256<D>(data: D) -> [u8; 32]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
	let mut dest = [0; 32];

	twox(&mut dest, data, 0);
	twox(&mut dest, data, 1);
	twox(&mut dest, data, 2);
	twox(&mut dest, data, 3);

	dest
}

/// Hash the data into `twox_64(data) + data`.
pub fn twox64_concat<D>(data: D) -> Vec<u8>
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();
	let mut v = twox64(data).to_vec();

	v.extend_from_slice(data);

	v
}

/// Hash the data into a 32-bytes array with Keccak algorithm.
pub fn keccak256<D>(data: D) -> [u8; 32]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();

	let mut keccak = Keccak::v256();
	keccak.update(data);

	let mut output = [0u8; 32];
	keccak.finalize(&mut output);

	output
}

/// Hash the data into a 64-bytes array with Keccak algorithm.
pub fn keccak512<D>(data: D) -> [u8; 64]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();

	let mut keccak = Keccak::v512();
	keccak.update(data);

	let mut output = [0u8; 64];
	keccak.finalize(&mut output);

	output
}

/// Hash the data into a 32-bytes array with SHA2 algorithm.
pub fn sha2_256<D>(data: D) -> [u8; 32]
where
	D: AsRef<[u8]>,
{
	let data = data.as_ref();

	let mut hasher = Sha256::new();
	hasher.update(data);

	let mut output = [0u8; 32];
	output.copy_from_slice(&hasher.finalize());

	output
}
