// --- std ---
use std::{fs, process::Command};

pub fn node_template(name: &str) {
	Command::new("git")
		.args(&[
			"clone",
			"https://github.com/substrate-developer-hub/substrate-node-template.git",
			name,
		])
		.output()
		.unwrap();
}

pub fn pallet_template(
	name: &str,
	multi_instance: bool,
	dependency_path: Option<&str>,
	dependency_git: Option<&str>,
	dependency_commit: Option<&str>,
	dependency_branch: Option<&str>,
	dependency_tag: Option<&str>,
) {
	Command::new("git")
		.args(&[
			"clone",
			"-b",
			if multi_instance {
				"multi-instance"
			} else {
				"single-instance"
			},
			"--single-branch",
			"https://github.com/l2ust/substrate-pallet-template.git",
			name,
		])
		.output()
		.unwrap();

	let path = |f| format!("{}/{}", name, f);
	let _ = fs::remove_file(path(".editorconfig"));
	let _ = fs::remove_file(path(".gitignore"));
	let _ = fs::remove_file(path(".rustfmt.toml"));
	let _ = fs::remove_file(path("Cargo.lock"));
	let _ = fs::remove_dir_all(path(".git"));
}
