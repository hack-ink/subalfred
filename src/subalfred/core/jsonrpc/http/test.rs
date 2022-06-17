// crates.io
use serde_json::Value;
// hack-ink
use super::*;
use subrpcer::system;

#[tokio::test]
async fn send_jsonrpc_should_work() {
	let response = send::<_, String>("https://rpc.polkadot.io", &system::chain_once()).await;

	assert!(response.is_ok());

	assert_eq!(response.unwrap().result, "Polkadot");
}
