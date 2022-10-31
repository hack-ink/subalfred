// crates.io
use clap::Args;
// hack-ink
use crate::{command::shared::TwoChainSpec, prelude::*};
use subalfred_core::state;

/// Check the differences between the two states.
/// Note:
/// This is not a symmetric difference operation.
/// `a.diff(b)` might equal `b.diff(a)`, but not always.
#[derive(Debug, Args)]
#[command(verbatim_doc_comment, override_usage = "subalfred state diff [OPTIONS] <PATH> <PATH>")]
pub(crate) struct DiffCmd {
	#[command(flatten)]
	two_state_config: TwoChainSpec,
}
impl DiffCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { two_state_config: TwoChainSpec { a, b } } = self;
		let diff = state::diff(a, b)?;

		if !diff.is_empty() {
			println!("{}", diff.join("\n"));
		}

		Ok(())
	}
}
