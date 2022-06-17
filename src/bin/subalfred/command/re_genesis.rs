// crates.io
use clap::Args;
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
	#[tokio::main]
	pub async fn run(&self) -> AnyResult<()> {
		let Self { live } = self;

		re_genesis::run(live).await?;

		Ok(())
	}
}
