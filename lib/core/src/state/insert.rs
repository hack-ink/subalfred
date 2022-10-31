//! Insert core library.

// hack-ink
use super::ChainSpec;
use crate::{prelude::*, system};

/// Insert the key/value pair into the specific file.
///
/// If the key already exists, it will be overwritten.
pub fn insert_pair_to_chain_spec(chain_spec_path: &str, key: String, value: String) -> Result<()> {
	let mut chain_spec = system::read_file_to_struct::<_, ChainSpec>(chain_spec_path)?;

	chain_spec.genesis.raw.top.insert(key, value);

	system::swap_file_data(
		chain_spec_path,
		&serde_json::to_vec(&chain_spec).map_err(error::Generic::Serde)?,
	)?;

	Ok(())
}
