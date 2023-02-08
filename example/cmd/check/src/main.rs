// subalfred
use se_util::ExampleNodeEnv;

fn main() {
	check_features();
	check_runtime();
}

#[allow(unused)]
fn check_features() {
	let ExampleNodeEnv { repository_dir, .. } = ExampleNodeEnv::setup(false);
	se_util::run(
		"subalfred",
		&["check", "features", "--manifest-path", &format!("{repository_dir}/runtime")],
	)
}

#[allow(unused)]
fn check_runtime() {
	let ExampleNodeEnv { executable_path, log_path, data_dir, .. } = ExampleNodeEnv::setup(true);
	let node = se_util::run_bg(&executable_path, &["-d", &data_dir, "--dev"], Some(&log_path));

	se_util::run(
		"subalfred",
		&[
			"check",
			"runtime",
			"--property",
			"version",
			"--executable",
			&executable_path,
			"--chain",
			"dev",
			"--live",
			"https://rpc.polkadot.io",
			// "https://rpc.darwinia.network",
		],
	);
	se_util::run(
		"subalfred",
		&[
			"check",
			"runtime",
			"--property",
			"storage",
			"--executable",
			&executable_path,
			"--chain",
			"dev",
			"--live",
			"https://rpc.polkadot.io",
			// "https://rpc.darwinia.network",
		],
	);

	// Let node keeps running on the background.
	//
	// Use ctrl-c to exit.
	node.wait();
}
