// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

// TODO: support `StorageNMap`
/// Calculate the storage key for the storage prefix/item.
#[derive(Debug, Args)]
pub struct StorageKeyCmd {
	/// Prefix of the storage.
	#[clap(long, required = true, value_name = "PREFIX")]
	prefix: String,
	/// Name of the storage item.
	#[clap(long, required = true, value_name = "ITEM")]
	item: String,
}
impl StorageKeyCmd {
	pub fn run(&self) -> Result<()> {
		let Self { prefix, item } = self;

		println!("{}", substorager::storage_key(prefix.as_bytes(), item.as_bytes()));

		Ok(())
	}
}
