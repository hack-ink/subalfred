// crates.io
use clap::Args;
use futures::executor;
// hack-ink
use crate::prelude::*;
use subalfred::core::node::re_genesis;

///
#[derive(Debug, Args)]
pub struct ReGenesisCmd {
	/// Live chain's RPC HTTP endpoint.
	#[clap(required = true, value_name = "")]
	live: String,
}
impl ReGenesisCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { live } = self;

		executor::block_on(re_genesis::run(live))?;

		Ok(())
	}
}
