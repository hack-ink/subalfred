// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred_core::state::{self, TwoStateConfig};

/// Check the diff between two states.
///
/// Note:
/// This is not a symmetric diff.
/// `a.diff(b)` may equals to `b.diff(a)`, but not always.
#[derive(Debug, Args)]
#[command(verbatim_doc_comment, override_usage = "subalfred state diff [OPTIONS] <PATH> <PATH>")]
pub(crate) struct DiffCmd {
	#[clap(flatten)]
	two_state_config: TwoStateConfig,
}
impl DiffCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { two_state_config: TwoStateConfig { a, b } } = self;
		let diff = state::diff(a, b)?;

		if !diff.is_empty() {
			println!("{}", diff.join("\n"));
		}

		Ok(())
	}
}
