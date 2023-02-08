// crates.io
use clap::Args;
// subalfred
use crate::prelude::*;
use subalfred_core::state::{self, ForkOffConfig};

/// Export the chain state from the Substrate-like node through the WS RPC endpoint.
///
/// The result will be stored at `<a>.export`.
#[derive(Debug, Args)]
pub(crate) struct ExportCmd {
	/// Live chain's HTTP RPC endpoint.
	#[arg(value_name = "URI")]
	live: String,
	/// Export the data starting from this block.
	///
	/// Accept block hash or block number.
	#[arg(long, value_name = "HASH/NUM")]
	at: Option<String>,
	/// Timeout for the fetching.
	#[arg(long, value_name = "SECS", default_value = "10")]
	timeout: u32,
	/// Export all the data.
	///
	/// So, it conflicts with any other filter option.
	///
	/// Note:
	/// The default behaviour (without this option) is fetching according to metadata's pallet
	/// storage records, which means if there is any old storage prefix that can not be found in
	/// the current runtime's pallet storage names will be ignored.
	#[arg(verbatim_doc_comment, long, conflicts_with_all = &["skip_pallets", "renew_consensus_with", "simple_governance"])]
	all: bool,
	/// Skip these pallets, while fetching.
	///
	/// It's useful when you want to skip the 'large' pallets.
	#[arg(long, value_delimiter = ',', value_name = "[PALLET]", conflicts_with = "all")]
	skip_pallets: Vec<String>,
	#[command(flatten)]
	fork_off_config: ForkOffConfig,
}
impl ExportCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { live, at, timeout, all, skip_pallets, fork_off_config } = self;

		state::export(live, at.to_owned(), *timeout, *all, skip_pallets, fork_off_config).await?;

		Ok(())
	}
}
