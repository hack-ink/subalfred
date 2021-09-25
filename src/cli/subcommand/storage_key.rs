// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::{cli::Run, AnyResult, Subalfred};

#[derive(Debug, StructOpt)]
#[structopt(help = "Calculate the storage key for the storage PREFIX/ITEM")]
pub struct StorageKeyCmd {
	#[structopt(
		short,
		long,
		required = true,
		takes_value = true,
		value_name = "PREFIX"
	)]
	prefix: String,
	#[structopt(short, long, takes_value = true, value_name = "ITEM")]
	item: Option<String>,
}
impl Run for StorageKeyCmd {
	fn run(&self) -> AnyResult<()> {
		let Self { prefix, item }: &StorageKeyCmd = self;
		println!(
			"{}",
			Subalfred::storage_key(prefix, item.as_ref().map(AsRef::as_ref)),
		);

		Ok(())
	}
}
