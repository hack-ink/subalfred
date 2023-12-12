// subalfred
use super::*;
use subrpcer::system;

#[tokio::test]
async fn send_jsonrpc_should_work() {
	let response =
		send::<_, String>("https://rpc.polkadot.io", &system::chain(0), Duration::from_secs(10))
			.await;

	assert!(response.is_ok());

	assert_eq!(response.unwrap().result, "Polkadot");
}
