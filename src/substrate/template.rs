// --- std ---
use std::process::Command;

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
	version: Option<&str>,
	path: Option<&str>,
	git: Option<&str>,
	commit: Option<&str>,
	branch: Option<&str>,
	tag: Option<&str>,
) {
}
