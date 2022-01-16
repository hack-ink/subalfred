// --- crates.io ---
use clap::Parser;
// --- hack-ink ---
use crate::*;

#[derive(Debug, Parser)]
pub struct NodeCmd {
	#[clap(
		help = "Path to the executable",
		short,
		long,
		required = true,
		takes_value = true,
		value_name = "PATH"
	)]
	executable: String,
	#[clap(
		help = "Chain name",
		short,
		long,
		required = true,
		takes_value = true,
		value_name = "NAME"
	)]
	chain: String,
	#[clap(
		help = "Remote chain RPC endpoint",
		short,
		long,
		required = true,
		takes_value = true,
		value_name = "URI"
	)]
	live_rpc_endpoint: String,
}
impl NodeCmd {
	pub fn check_runtime_version(&self) -> AnyResult<()> {
		Executor::check_runtime_version(&self.executable, &self.chain, &self.live_rpc_endpoint)
	}

	pub fn check_storage_prefix(&self) -> AnyResult<()> {
		Executor::check_storage_prefix(&self.executable, &self.chain, &self.live_rpc_endpoint)
	}
}
