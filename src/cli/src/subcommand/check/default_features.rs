// --- std ---
use std::path::PathBuf;
// --- crates.io ---
use clap::Parser;
// --- hack-ink ---
use crate::*;

#[derive(Debug, Parser)]
pub struct DefaultFeaturesCmd {
	#[clap(short, long, required = true, takes_value = true)]
	project_dir: PathBuf,
}
impl Run for DefaultFeaturesCmd {
	fn run(&self) -> AnyResult<()> {
		Executor::check_default_features(&self.project_dir)
	}
}
