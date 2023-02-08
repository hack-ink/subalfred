// crates.io
use clap::Args;
// subalfred
use crate::prelude::*;

// TODO: support `StorageNMap`
/// Calculate the storage key of the storage item.
#[derive(Debug, Args)]
pub(crate) struct StorageKeyCmd {
	/// Prefix of the storage.
	#[arg(long, required = true, value_name = "PALLET")]
	pallet: String,
	/// Name of the storage item.
	#[arg(long, required = true, value_name = "ITEM")]
	item: String,
}
impl StorageKeyCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { pallet, item } = self;

		println!("{}", substorager::storage_key(pallet.as_bytes(), item.as_bytes()));

		Ok(())
	}
}
