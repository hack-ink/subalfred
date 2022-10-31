//! Fork-off core library.

// hack for
use super::ChainSpec;
use crate::{prelude::*, system};

/// Insert the key/value pair into the specific file.
pub fn insert_paris_to_chain_spec(chain_spec_path: &str, key: String, value: String) -> Result<()> {
	let mut chain_spec = system::read_file_to_struct::<_, ChainSpec>(chain_spec_path)?;

	chain_spec.genesis.raw.top.insert(key, value);

	system::write_data_to_file(
		chain_spec_path,
		&serde_json::to_vec(&chain_spec).map_err(error::Generic::Serde)?,
	)?;

	Ok(())
}
