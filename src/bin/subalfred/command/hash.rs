// std
use std::borrow::Cow;
// crates.io
use clap::{ArgEnum, Args};
// hack-ink
use crate::prelude::*;

/// Hash the hex with the specific hasher.
#[derive(Debug, Args)]
pub struct HashCmd {
	/// Value to be hashed.
	#[clap(required = true, value_name = "HEX/BINARY STRING")]
	value: String,
	/// Hash algorithm.
	#[clap(arg_enum, long, value_name = "HASHER", default_value = "blake2-128-concat")]
	hasher: Hasher,
	/// Read value into binary string format.
	#[clap(long, takes_value = false)]
	bstring: bool,
}
impl HashCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { value, hasher, bstring } = self;
		let data = if *bstring {
			Cow::Borrowed(value.as_bytes())
		} else {
			Cow::Owned(array_bytes::hex2bytes(value).map_err(quick_error)?)
		};
		let bytes = match hasher {
			Hasher::blake2_128 => subhasher::blake2_128(&data).to_vec(),
			Hasher::blake2_128_concat => subhasher::blake2_128_concat(&data),
			Hasher::blake2_256 => subhasher::blake2_256(&data).to_vec(),
			Hasher::blake2_512 => subhasher::blake2_512(&data).to_vec(),
			Hasher::twox64 => subhasher::twox64(&data).to_vec(),
			Hasher::twox64_concat => subhasher::twox64_concat(&data),
			Hasher::twox128 => subhasher::twox128(&data).to_vec(),
			Hasher::twox256 => subhasher::twox256(&data).to_vec(),
			Hasher::keccak256 => subhasher::keccak256(&data).to_vec(),
			Hasher::keccak512 => subhasher::keccak512(&data).to_vec(),
			Hasher::sha2_256 => subhasher::sha2_256(&data).to_vec(),
		};

		println!("{}", array_bytes::bytes2hex("0x", &bytes));

		Ok(())
	}
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, ArgEnum)]
pub enum Hasher {
	blake2_128,
	blake2_128_concat,
	blake2_256,
	blake2_512,
	twox64,
	twox64_concat,
	twox128,
	twox256,
	keccak256,
	keccak512,
	sha2_256,
}
