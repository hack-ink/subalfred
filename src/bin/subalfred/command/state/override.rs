// std
use std::path::PathBuf;
// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::state;

/// Override state a with b.
#[derive(Debug, Args)]
#[clap(verbatim_doc_comment, usage = "subalfred state merge [OPTIONS] <PATH> <PATH>")]
pub(crate) struct OverrideCmd {
	#[clap(required = true, value_name = "PATH")]
	a: PathBuf,
	#[clap(required = true, value_name = "PATH")]
	b: PathBuf,
}
impl OverrideCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { a, b } = self;

		Ok(state::r#override(a, b)?)
	}
}
