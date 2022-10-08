// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::state::{self, ForkOffConfig};

/// Export the chain state.
///
/// The result will be store at `<a>.export`.
#[derive(Debug, Args)]
pub(crate) struct ExportCmd {
	/// Live chain's RPC HTTP endpoint.
	#[arg(required = true, value_name = "URI")]
	live: String,
	/// Export the data starting from this block.
	#[arg(long, value_name = "HASH")]
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
	#[arg(verbatim_doc_comment, long, num_args = 0, conflicts_with_all = &["skip-pallets", "renew-consensus-with", "simple-governance"])]
	all: bool,
	/// Skip these pallets, while fetching.
	///
	/// It's useful when you want to skip the 'large' pallets.
	#[arg(long, value_delimiter = ',', value_name = "[PALLET_NAME]", conflicts_with = "all")]
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
