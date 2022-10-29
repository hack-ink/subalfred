// hack-ink
use super::*;
use subrpcer::system;

#[tokio::test]
async fn send_jsonrpc_should_work() {
	let response = send::<_, String>("https://rpc.polkadot.io", &system::chain(0)).await;

	assert!(response.is_ok());

	assert_eq!(response.unwrap().result, "Polkadot");
}
