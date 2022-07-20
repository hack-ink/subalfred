// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::node::export_state::{self, Config};

/// Export the chain state.
#[derive(Debug, Args)]
pub struct ExportStateCmd {
	/// Live chain's RPC HTTP endpoint.
	#[clap(required = true, value_name = "URI")]
	live: String,
	/// Fetch the data starting from this block.
	#[clap(long, value_name = "HASH")]
	at: Option<String>,
	#[clap(flatten)]
	config: Config,
}
impl ExportStateCmd {
	#[tokio::main]
	pub async fn run(&self) -> Result<()> {
		let Self { live, at, config } = self;

		export_state::run(live, at.clone(), config).await?;

		Ok(())
	}
}
