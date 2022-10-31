// crates.io
use clap::Args;
use serde_json::Value;
// hack-ink
use crate::prelude::*;
use subalfred_core::http::CLIENT;

/// Send a RPC request to the node's HTTP RPC endpoint.
///
/// Example:
/// Get the Polkadot's block zero's hash:
/// ```
/// # Normal output
/// subalfred rpc https://rpc.polkadot.io --method chain_getBlockHash --params '[[0,1,2]]'
/// # Beautiful output
/// subalfred rpc https://rpc.polkadot.io --method chain_getBlockHash --params '[[0,1,2]]' | jq
/// ```
#[derive(Debug, Args)]
pub(crate) struct RpcCmd {
	/// Node's HTTP RPC endpoint.
	#[arg(required = true, value_name = "URI", default_value = "http://localhost:9933")]
	uri: String,
	/// JSONRPC method name.
	#[arg(long, required = true, value_name = "METHOD")]
	method: String,
	/// JSONRPC parameters.
	#[arg(long, value_name = "[PARAMETER]")]
	params: Option<String>,
}
impl RpcCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { method, params, uri } = self;
		let params = if let Some(params) = params.as_ref() {
			serde_json::from_str(params)?
		} else {
			Value::Null
		};
		let result = CLIENT
			.post(uri)
			.json(&subrpcer::rpc(0, method, params))
			.send()
			.await?
			.json::<Value>()
			.await?;
		let result = serde_json::to_string(&result)?;

		println!("{result}");

		Ok(())
	}
}
