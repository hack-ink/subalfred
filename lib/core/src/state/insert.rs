//! Insert core library.

// hack-ink
use super::ChainSpec;
use crate::{prelude::*, system};

/// Insert the key/value pair into the specific file.
///
/// If the key already exists, it will be overwritten.
///
/// # Examples
/// ```sh
/// # Calculate the WASM code key.
/// subalfred convert ascii2hex ':code'
/// # "0x3a636f6465"
/// # Override the WASM code.
/// subalfred state insert genesis.json --key 0x3a636f6465 --with-file runtime.compact.compressed.wasm
/// ```
pub fn insert_paris_to_chain_spec(chain_spec_path: &str, key: String, value: String) -> Result<()> {
	let mut chain_spec = system::read_file_to_struct::<_, ChainSpec>(chain_spec_path)?;

	chain_spec.genesis.raw.top.insert(key, value);

	system::swap_file_data(
		chain_spec_path,
		&serde_json::to_vec(&chain_spec).map_err(error::Generic::Serde)?,
	)?;

	Ok(())
}
