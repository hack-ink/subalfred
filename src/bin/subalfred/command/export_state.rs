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
	/// Output to this URI.
	///
	/// If the given URI is a HTTP address. It will be downloaded first.
	#[clap(long, value_name = "URI", default_value = "exported-state.json")]
	pub output: String,
	#[clap(long, takes_value = false)]
	pub renew_authorities: bool,
	#[clap(long, takes_value = false)]
	pub renew_government: bool,
	// TODO
	// pub renew_runtime_code: bool,
}
impl Into<ExportConfig> for &ExportConfigArgs {
	fn into(self) -> ExportConfig {
		ExportConfig {
			path: self.output.clone(),
			renew_authorities: self.renew_authorities.clone(),
			renew_government: self.renew_government.clone(),
			// TODO
			// renew_runtime_code: self.renew_runtime_code,
		}
	}
}
