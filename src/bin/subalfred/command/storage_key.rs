// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

/// Calculate the storage key for the storage PREFIX/ITEM.
#[derive(Debug, Args)]
pub struct StorageKeyCmd {
	/// Prefix of the storage.
	#[clap(long, required = true, value_name = "PREFIX")]
	prefix: String,
	/// Name of the storage item.
	#[clap(long, value_name = "ITEM")]
	item: Option<String>,
}
impl StorageKeyCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { prefix, item } = self;

		println!(
			"{}",
			array_bytes::bytes2hex(
				"0x",
				substorager::storage_key(
					prefix.as_ref(),
					item.as_ref().map(|s| s.as_str()).unwrap_or_default().as_ref()
				)
			)
		);

		Ok(())
	}
}
