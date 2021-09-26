mod default_features;
use default_features::DefaultFeaturesCmd;

mod node;
use node::NodeCmd;

// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::{cli::Run, AnyResult};

#[derive(Debug, StructOpt)]
pub enum CiCmd {
	DefaultFeatures(DefaultFeaturesCmd),
	RuntimeVersion(NodeCmd),
	StoragePrefix(NodeCmd),
}
impl Run for CiCmd {
	// TODO use macro
	fn run(&self) -> AnyResult<()> {
		match self {
			CiCmd::DefaultFeatures(cmd) => cmd.run(),
			CiCmd::RuntimeVersion(cmd) => cmd.run_runtime_version_check(),
			CiCmd::StoragePrefix(cmd) => cmd.run_storage_prefix_check(),
		}
	}
}
