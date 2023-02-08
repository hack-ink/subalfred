// subalfred
use super::*;

#[tokio::test]
async fn runtime_version_should_work() {
	assert_eq!(
		runtime_version(
			"https://rpc.polkadot.io",
			Some("0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"),
		)
		.await
		.unwrap(),
		RuntimeVersion {
			spec_name: "polkadot".into(),
			impl_name: "parity-polkadot".into(),
			authoring_version: 0,
			spec_version: 0,
			impl_version: 0,
			transaction_version: 0,
			state_version: 0
		}
	);
}
