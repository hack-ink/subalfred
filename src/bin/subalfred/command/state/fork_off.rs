// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::state::{self, ForkOffConfig};

/// Fork-off the state.
///
/// The result will be store at `<a>.fork-off`.
#[derive(Debug, Args)]
pub(crate) struct ForkOffCmd {
	path: String,
	#[clap(flatten)]
	fork_off_config: ForkOffConfig,
}
impl ForkOffCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { path, fork_off_config } = self;

		state::fork_off(path, fork_off_config)?;

		Ok(())
	}
}
