// crates.io
use clap::Args;
// subalfred
use crate::{command::shared::TwoChainSpec, prelude::*};
use subalfred_core::state;

/// Override the chain spec a with b.
///
/// The result will be stored at `<a>.override`.
#[derive(Debug, Args)]
#[command(
	verbatim_doc_comment,
	override_usage = "subalfred state override [OPTIONS] <PATH> <PATH>"
)]
pub(crate) struct OverrideCmd {
	#[command(flatten)]
	two_state_config: TwoChainSpec,
}
impl OverrideCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { two_state_config: TwoChainSpec { a, b } } = self;

		Ok(state::r#override(a, b)?)
	}
}
