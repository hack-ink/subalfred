// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

/// TODO.
#[derive(Debug, Args)]
pub(crate) struct GetCmd {
	/// Live chain's RPC HTTP endpoint.
	#[clap(required = true, value_name = "URI")]
	live: String,
}
impl GetCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { live } = self;

		Ok(())
	}
}
