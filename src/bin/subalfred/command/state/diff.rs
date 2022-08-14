// std
use std::path::PathBuf;
// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::state;

/// Check the diff between two states.
///
/// Note:
/// This is not a symmetric diff.
/// `a.diff(b)` may equals to `b.diff(a)`, but not always.
#[derive(Debug, Args)]
#[clap(verbatim_doc_comment, usage = "subalfred state diff [OPTIONS] <PATH> <PATH>")]
pub(crate) struct DiffCmd {
	#[clap(required = true, value_name = "PATH")]
	a: PathBuf,
	#[clap(required = true, value_name = "PATH")]
	b: PathBuf,
}
impl DiffCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { a, b } = self;
		let diff = state::diff(a, b)?;

		if !diff.is_empty() {
			println!("{}", diff.join("\n"));
		}

		Ok(())
	}
}
