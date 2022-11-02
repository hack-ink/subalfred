// std
use std::fs;
// crates.io
use se_util::ExampleNodeEnv;
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
	export();
	// Build the node-template genesis first.
	// Then we could compare it with the exported one.
	// /tmp/subalfred-example/substrate-node-template/target/debug/node-template build-spec --dev
	// --raw > /tmp/subalfred-example/chain-spec.json
	diff();
	fork_off();
}

fn export() {
	let ExampleNodeEnv { base_dir, executable_path, log_path, data_dir, .. } =
		ExampleNodeEnv::setup(false);
	let node = se_util::run_bg(&executable_path, &["-d", &data_dir, "--dev"], Some(&log_path));

	// Make sure the node was fully boot up.
	se_util::sleep(6000);
	// Make some changes to the storage.
	//
	// Transfer one coin from Alice to Dave.
	Runtime::new().unwrap().block_on(async {
		let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
		let signer = PairSigner::new(AccountKeyring::Alice.pair());
		let dest = AccountKeyring::Dave.to_account_id().into();
		let tx = runtime::tx().balances().transfer(dest, 1_000_000_000_000);

		api.tx().sign_and_submit_default(&tx, &signer).await.unwrap();
	});
	// Make sure the extrinsic has been finished.
	se_util::sleep(12000);
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

	// Default name.
	let export_file = "default-chain-spec.json.export";

	fs::rename(export_file, format!("{base_dir}/{export_file}")).unwrap();

	node.kill();
}

fn diff() {
	se_util::run(
		"subalfred",
		&[
			"state",
			"diff",
			"/tmp/subalfred-example/chain-spec.json",
			"/tmp/subalfred-example/default-chain-spec.json.export",
		],
	);
}

fn fork_off() {
	se_util::run(
		"subalfred",
		&[
			"state",
			"fork-off",
			"/tmp/subalfred-example/default-chain-spec.json.export",
			"--renew-consensus-with",
			"/tmp/subalfred-example/chain-spec.json",
			"--disable-default-bootnodes",
		],
	);
}
