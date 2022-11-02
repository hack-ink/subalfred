// crates.io
use clap::{ArgGroup, Args};
// hack-ink
use crate::prelude::*;
use subalfred_core::{state, system};

/// Insert the key/value pair into the specific file.
///
/// If the key already exists, it will be overwritten.
///
/// # Example
/// ```sh
/// # Calculate the WASM code key.
/// subalfred convert ascii2hex ':code'
/// # "0x3a636f6465"
/// # Override the WASM code.
/// subalfred state insert chain-spec.json --key 0x3a636f6465 --with-file runtime.compact.compressed.wasm
/// ```
#[derive(Debug, Args)]
#[command(group(
	ArgGroup::new("vers")
		.required(true)
		.args(["value", "with_file"]),
))]
pub(crate) struct InsertCmd {
	/// Target state file's path.
	#[arg(required = true, value_name = "PATH")]
	path: String,
	/// Storage key.
	#[arg(long, required = true, value_name = "HEX")]
	key: String,
	/// Storage value.
	#[arg(long, value_name = "HEX")]
	value: Option<String>,
	/// Storage value file.
	#[arg(long, value_name = "PATH")]
	with_file: Option<String>,
}
impl InsertCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { path, key, value, with_file } = self;
		let value = if let Some(value) = value {
			value.to_owned()
		} else if let Some(with_file) = with_file {
			let bytes = system::read_file_to_vec(with_file)?;

			array_bytes::bytes2hex("0x", &bytes)
		} else {
			Default::default()
		};

		state::insert_pair_to_chain_spec(path, key.to_owned(), value)?;

		Ok(())
	}
}
