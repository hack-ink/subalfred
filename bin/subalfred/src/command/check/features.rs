// crates.io
use clap::Args;
// subalfred
use crate::prelude::*;

/// Please use cargo-featalign instead, which can be found at https://github.com/hack-ink/cargo-featalign.
#[derive(Debug, Args)]
#[command(verbatim_doc_comment)]
pub(crate) struct FeaturesCmd {}
impl FeaturesCmd {
	pub(crate) fn run(&self) -> Result<()> {
		Ok(())
	}
}
