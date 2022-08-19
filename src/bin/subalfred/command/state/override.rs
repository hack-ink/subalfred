// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::state::{self, TwoStateConfig};

/// Override state a with b.
///
/// The result will be store at `<a-file-name>.override`.
#[derive(Debug, Args)]
#[clap(verbatim_doc_comment, usage = "subalfred state merge [OPTIONS] <PATH> <PATH>")]
pub(crate) struct OverrideCmd {
	#[clap(flatten)]
	two_state_config: TwoStateConfig,
}
impl OverrideCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { two_state_config: TwoStateConfig { a, b } } = self;

		Ok(state::r#override(a, b)?)
	}
}
