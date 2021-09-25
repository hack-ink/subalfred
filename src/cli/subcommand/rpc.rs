// --- crates.io ---
use isahc::ReadResponseExt;
use serde_json::Value;
use structopt::StructOpt;
use subrpcer::client::i::{self};
// --- subalfred ---
use crate::{cli::Run, AnyResult, Subalfred};

#[derive(Debug, StructOpt)]
pub struct RpcCmd {
	#[structopt(
		short,
		long,
		takes_value = true,
		value_name = "URI",
		default_value = "http://localhost:9933"
	)]
	uri: String,
	#[structopt(
		help = "Support these styles (non case sensitive):\n\
				\t- state_getRuntimeVersion\n\
				\t- state_getruntimeversion\n\
				\t- getRuntimeVersion\n\
				\t- getruntimeversion",
		short,
		long,
		required = true,
		takes_value = true,
		value_name = "METHOD"
	)]
	method: String,
	#[structopt(
		short,
		long,
		takes_value = true,
		value_name = "[PARAM]",
		default_value = "[]"
	)]
	params: Value,
	#[structopt(
		short,
		long,
		takes_value = true,
		value_name = "ID",
		default_value = "1"
	)]
	id: u16,
}
impl Run for RpcCmd {
	fn run(&self) -> AnyResult<()> {
		let Self {
			uri,
			method,
			params,
			id,
		}: &RpcCmd = self;
		let rpc = match self.method.to_lowercase().as_str() {
			// TODO
			// "author_submitandwatchextrinsic" | "submitandwatchextrinsic" => {
			// subrpcer::author::submit_and_watch_extrinsic_with_raw_params_and_id(id, params)
			// }
			"chain_getblockhash" | "getblockhash" => {
				subrpcer::chain::get_block_hash_with_raw_params_and_id(id, params)
			}
			"state_getruntimeversion" | "getruntimeversion" => {
				subrpcer::state::get_runtime_version_with_id(id)
			}
			"state_getmetadata" | "getmetadata" => subrpcer::state::get_metadata_with_id(id),
			"state_getstorage" | "getstorage" => {
				subrpcer::state::get_storage_with_raw_params_and_id(id, params)
			}
			_ => subrpcer::rpc(id, method, params),
		};

		tracing::trace!("{}", serde_json::to_string_pretty(&rpc)?);
		println!("{}", i::send_rpc(uri, rpc)?.text()?);

		Ok(())
	}
}
