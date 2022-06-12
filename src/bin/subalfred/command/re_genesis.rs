// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

///
#[derive(Debug, Args)]
pub struct ReGenesisCmd {
	///
	#[clap(long, required = true, value_name = "")]
	_todo: String,
}
impl ReGenesisCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { _todo } = self;

		Ok(())
	}
}
