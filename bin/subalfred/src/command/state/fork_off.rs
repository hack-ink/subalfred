// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred_core::state::{self, ForkOffConfig};

/// Fork-off the Substrate-like chain state.
///
/// The result will be stored at `<a>.fork-off`.
#[derive(Debug, Args)]
pub(crate) struct ForkOffCmd {
	/// Target state file's path.
	#[arg(required = true, value_name = "PATH")]
	path: String,
	#[command(flatten)]
	fork_off_config: ForkOffConfig,
}
impl ForkOffCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { path, fork_off_config } = self;

		state::fork_off(path, fork_off_config)?;

		Ok(())
	}
}
