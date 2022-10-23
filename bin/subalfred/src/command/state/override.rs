// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred_core::state::{self, TwoStateConfig};

/// Override state a with b.
///
/// The result will be store at `<a>.override`.
#[derive(Debug, Args)]
#[command(verbatim_doc_comment, override_usage = "subalfred state merge [OPTIONS] <PATH> <PATH>")]
pub(crate) struct OverrideCmd {
	#[command(flatten)]
	two_state_config: TwoStateConfig,
}
impl OverrideCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { two_state_config: TwoStateConfig { a, b } } = self;

		Ok(state::r#override(a, b)?)
	}
}
