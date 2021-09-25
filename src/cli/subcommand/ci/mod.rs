mod default_features;
use default_features::DefaultFeaturesCmd;

mod storage_prefix;
use storage_prefix::StoragePrefixCmd;

// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::{cli::Run, AnyResult, Subalfred};

#[derive(Debug, StructOpt)]
pub enum CiCmd {
	DefaultFeatures(DefaultFeaturesCmd),
	StoragePrefix(StoragePrefixCmd),
}
impl Run for CiCmd {
	// TODO use macro
	fn run(&self) -> AnyResult<()> {
		match self {
			CiCmd::DefaultFeatures(cmd) => cmd.run(),
			CiCmd::StoragePrefix(cmd) => cmd.run(),
		}
	}
}
