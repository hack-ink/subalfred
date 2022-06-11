// crates.io
use serde_json::Value;
// hack-ink
use super::*;
use subrpcer::system;

#[async_std::test]
async fn send_jsonrpc_should_work() {
	let result = send_jsonrpc("https://rpc.polkadot.io", &system::chain()).await;

	assert!(result.is_ok());

	let result = result.unwrap().json::<Value>().await;

	assert!(result.is_ok());
	assert_eq!(result.unwrap()["result"].as_str().unwrap(), "Polkadot");
}
