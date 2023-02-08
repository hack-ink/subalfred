// crates.io
use clap::Args;
// subalfred
use crate::prelude::*;
use subalfred_core::node;

/// Get the runtime upgrade block.
///
/// Using the dichotomy algorithm to find it out.
/// This operation will fail if the runtime version does not existed.
#[derive(Debug, Args)]
pub(crate) struct RuntimeUpgradeBlockCmd {
	/// At this runtime version.
	#[arg(value_name = "VERSION")]
	runtime_version: u32,
	/// Node's WS RPC endpoint.
	#[arg(long, required = true, value_name = "URI", default_value = "ws://localhost:9944")]
	uri: String,
}
impl RuntimeUpgradeBlockCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { runtime_version, uri } = self;

		if let Some((number, hash)) =
			node::find_runtime_upgrade_block(*runtime_version, uri).await?
		{
			println!("{number} {hash}");
		} else {
			println!("target runtime version not found");
		}

		Ok(())
	}
}
