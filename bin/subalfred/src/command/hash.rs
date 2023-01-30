// crates.io
use clap::{Args, ValueEnum};
// hack-ink
use crate::prelude::*;

/// Hash the hex with the specific hasher(hash algorithm).
#[derive(Debug, Args)]
pub(crate) struct HashCmd {
	/// Hex data input.
	#[arg(value_name = "HEX")]
	hex: String,
	/// Hash algorithm.
	#[arg(value_enum, long, value_name = "HASHER", default_value = "blake2-128-concat")]
	hasher: Hasher,
}
impl HashCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { hex, hasher } = self;
		let data = array_bytes::hex2bytes(hex).map_err(quick_err)?;
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

		println!("{}", array_bytes::bytes2hex("0x", bytes));

		Ok(())
	}
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, ValueEnum)]
enum Hasher {
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
