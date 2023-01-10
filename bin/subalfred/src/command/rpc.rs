// crates.io
use clap::Args;
use serde_json::Value;
// hack-ink
use crate::prelude::*;
use subalfred_core::http::CLIENT;

/// Send a RPC request to the node's HTTP RPC endpoint.
///
/// # Example:
/// Get the Polkadot's block zero's hash:
/// ```sh
/// # Normal output
/// subalfred rpc https://rpc.polkadot.io --method chain_getBlockHash --params '[[0,1,2]]'
/// # Beautiful output
/// subalfred rpc https://rpc.polkadot.io --method chain_getBlockHash --params '[[0,1,2]]' | jq
/// ```
#[derive(Debug, Args)]
#[command(verbatim_doc_comment)]
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
	async fn run_(&self) -> Result<String> {
		let Self { method, params, uri } = self;
		let params = if let Some(params) = params.as_ref() {
			serde_json::from_str(params)?
		} else {
			Value::Null
		};
		let r = CLIENT
			.post(uri)
			.json(&subrpcer::rpc(0, method, params))
			.send()
			.await?
			.json::<Value>()
			.await?;

		Ok(serde_json::to_string(&r)?)
	}

	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let r = self.run_().await?;

		println!("{r}");

		Ok(())
	}
}

#[tokio::test]
async fn rpc_cmd_should_work() {
	let cmd = RpcCmd {
		uri: "https://rpc.polkadot.io".into(),
		method: "chain_getBlockHash".into(),
		params: Some("[[0, 1, 2]]".into()),
	};

	assert_eq!(
		cmd.run_().await.unwrap(),
		"{\
			\"id\":0,\
			\"jsonrpc\":\"2.0\",\
			\"result\":[\
				\"0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3\",\
				\"0xc0096358534ec8d21d01d34b836eed476a1c343f8724fa2153dc0725ad797a90\",\
				\"0x409d0bfe677594d7558101d574633d5808a6fc373cbd964ef236f00941f290ee\"\
			]\
		}"
	);
}
