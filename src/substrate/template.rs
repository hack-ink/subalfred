// --- std ---
use std::{
	fs::{self, OpenOptions},
	io::{Read, Seek, SeekFrom, Write},
	process::Command,
};

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

	fs::remove_file(path(".editorconfig")).unwrap();
	fs::remove_file(path(".gitignore")).unwrap();
	fs::remove_file(path(".rustfmt.toml")).unwrap();
	fs::remove_file(path("Cargo.lock")).unwrap();
	fs::remove_dir_all(path(".git")).unwrap();

	let mut cargo_toml = OpenOptions::new()
		.read(true)
		.write(true)
		.open(path("Cargo.toml"))
		.unwrap();
	let mut cargo_toml_content = String::new();

	cargo_toml.read_to_string(&mut cargo_toml_content).unwrap();

	let lines_to_modify = [17, 18, 22, 23, 24];
	let dependency_extra_info = if let Some(dependency_path) = dependency_path {
		format!(", path = \"{}\"", dependency_path)
	} else if let Some(dependency_git) = dependency_git {
		let dependency_commit = if let Some(dependency_commit) = dependency_commit {
			format!(", commit = \"{}\"", dependency_commit)
		} else {
			String::new()
		};
		let dependency_branch = if let Some(dependency_branch) = dependency_branch {
			format!(", branch = \"{}\"", dependency_branch)
		} else {
			String::new()
		};
		let dependency_tag = if let Some(dependency_tag) = dependency_tag {
			format!(", tag = \"{}\"", dependency_tag)
		} else {
			String::new()
		};

		format!(
			", git = \"{}\"{}{}{}",
			dependency_git, dependency_commit, dependency_branch, dependency_tag
		)
	} else {
		String::new()
	};

	cargo_toml_content = cargo_toml_content
		.lines()
		.enumerate()
		.map(|(i, line)| {
			if lines_to_modify.contains(&(i + 1)) {
				format!(
					"{}{} }}\n",
					line.trim_end_matches(" }"),
					dependency_extra_info
				)
			} else {
				format!("{}\n", line)
			}
		})
		.collect();

	cargo_toml.seek(SeekFrom::Start(0)).unwrap();
	cargo_toml.write_all(cargo_toml_content.as_bytes()).unwrap();
	cargo_toml.sync_all().unwrap();
}
