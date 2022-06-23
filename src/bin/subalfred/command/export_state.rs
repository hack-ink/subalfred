// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::node::export_state::{self, ExportConfig};

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
	export_config_args: ExportConfigArgs,
}
impl ExportStateCmd {
	#[tokio::main]
	pub async fn run(&self) -> AnyResult<()> {
		let Self { live, at, export_config_args } = self;

		export_state::run(live, at.clone(), export_config_args.into()).await?;

		Ok(())
	}
}

#[derive(Debug, Args)]
pub struct ExportConfigArgs {
	/// Save the exported result to.
	#[clap(long, value_name = "PATH", default_value = "exported-state.json")]
	pub output: String,
	/// Fetch the data according to metadata's pallet storage records.
	///
	/// This means if there is any old storage prefix that can not be found in the current
	/// runtime's pallet storage names will be ignored.
	#[clap(long, takes_value = false)]
	pub from_metadata: bool,
	/// Skip exporting the authority related storages.
	#[clap(long, takes_value = false)]
	pub skip_authority: bool,
	/// Skip exporting the collective and sudo related storages.
	#[clap(long, takes_value = false)]
	pub skip_collective: bool,
}
impl Into<ExportConfig> for &ExportConfigArgs {
	fn into(self) -> ExportConfig {
		ExportConfig {
			output: self.output.clone(),
			from_metadata: self.from_metadata.clone(),
			skip_authority: self.skip_authority.clone(),
			skip_collective: self.skip_collective.clone(),
		}
	}
}
