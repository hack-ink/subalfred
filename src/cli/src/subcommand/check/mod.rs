mod default_features;
use default_features::DefaultFeaturesCmd;

mod node;
use node::NodeCmd;

// crates.io
use clap::Parser;
// hack-ink
use crate::{AnyResult, Run};

#[derive(Debug, Parser)]
pub enum CheckCmd {
	DefaultFeatures(DefaultFeaturesCmd),
	RuntimeVersion(NodeCmd),
	StoragePrefix(NodeCmd),
}
impl Run for CheckCmd {
	// TODO: use macro
	fn run(&self) -> AnyResult<()> {
		match self {
			CheckCmd::DefaultFeatures(cmd) => cmd.run(),
			CheckCmd::RuntimeVersion(cmd) => cmd.check_runtime_version(),
			CheckCmd::StoragePrefix(cmd) => cmd.check_storage_prefix(),
		}
	}
}
