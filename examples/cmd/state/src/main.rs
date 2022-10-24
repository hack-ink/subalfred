// std
use std::{env, path::Path};
// crates.io
use sp_keyring::AccountKeyring;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig};
use tokio::runtime::Runtime;

// ```sh
// /tmp/subalfred-example/substrate-node-template/target/debug/node-template --dev
// cargo install subxt-cli
// subxt metadata -f bytes > /tmp/subalfred-example/metadata.scale
// ```
#[subxt::subxt(runtime_metadata_path = "/tmp/subalfred-example/metadata.scale")]
pub mod runtime {}

fn main() {
	let base_dir = "/tmp/subalfred-example";
	let repository_dir = format!("{base_dir}/substrate-node-template");
	let executable_path = format!("{repository_dir}/target/debug/node-template");
	let log_path = format!("{base_dir}/log");
	let data_dir = format!("{base_dir}/data");

	if !Path::new(&repository_dir).exists() {
		// Clone the repository.
		se_util::run(
			"git",
			&[
				"clone",
				"https://github.com/substrate-developer-hub/substrate-node-template.git",
				&repository_dir,
			],
		);
	}

	env::set_current_dir(repository_dir).unwrap();
	// Build the node template.
	//
	// Make sure you have met the compiling requirement.
	// Such as, `gcc`, `llvm`, `wasm32-unknown-unknown`, `protobuf` and etc.
	se_util::run("cargo", &["build"]);

	let node = se_util::run_bg(&executable_path, &["-d", &data_dir, "--dev"], Some(&log_path));

	// Give some time for the node to boot up.
	se_util::sleep(6000);
	Runtime::new().unwrap().block_on(async {
		let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
		let signer = PairSigner::new(AccountKeyring::Alice.pair());
		let dest = AccountKeyring::Dave.to_account_id().into();
		let tx = runtime::tx().balances().transfer(dest, 1_000_000_000_000);

		api.tx().sign_and_submit_default(&tx, &signer).await.unwrap();
	});
	// Make sure the extrinsic has been finished.
	se_util::sleep(6000);
	// Export the chain state with Subalfred.
	se_util::run(
		"subalfred",
		&[
			"state",
			"export",
			"ws://127.0.0.1:9944",
			// "--renew-consensus-with"
			// "darwinia-dev.json",
			"--simple-governance",
			"--disable-default-bootnodes",
			"-lsubalfred_core::state,subalfred_core::substrate_client",
		],
	);

	// Let node keeps running on the background.
	//
	// Use ctrl-c to exit.
	node.wait();
}
