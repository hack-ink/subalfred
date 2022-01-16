// --- std ---
use std::{path::PathBuf, process};
// --- crates.io ---
use cargo_toml::Manifest;
use walkdir::WalkDir;
// --- hack-ink ---
use crate::*;

impl Executor {
	pub fn check_default_features(project_dir: &PathBuf) -> AnyResult<()> {
		let dir_name = project_dir
			.file_name()
			.ok_or(anyhow::anyhow!(""))?
			.to_str()
			.ok_or(anyhow::anyhow!(""))?;
		let mut optional_maybe_incomplete_dependencies = vec![];
		let mut incomplete_dependencies = vec![];

		for entry in WalkDir::new(&project_dir)
			.into_iter()
			.filter_entry(|entry| {
				let name = entry.file_name().to_str().unwrap();

				name != "target" && !name.starts_with('.') && !name.starts_with("./")
			})
			.filter_map(|entry| entry.ok())
		{
			if entry.file_name() == "Cargo.toml" {
				let manifest = Manifest::from_path(entry.path()).unwrap();

				if let Some(std) = manifest.features.get("std") {
					for (alias, dependency) in manifest.dependencies {
						if let Some(detail) = dependency.detail() {
							if let Some(default_features) = detail.default_features {
								if !default_features {
									if !std.contains(&format!("{}/std", alias)) {
										let dependency = (
											alias.to_owned(),
											entry
												.path()
												.to_str()
												.unwrap()
												.splitn(2, dir_name)
												.last()
												.unwrap()
												.to_owned(),
										);

										if detail.optional {
											optional_maybe_incomplete_dependencies.push(dependency);
										} else {
											incomplete_dependencies.push(dependency);
										}
									}
								}
							}
						}
					}
				}
			}
		}

		for (alias, path) in optional_maybe_incomplete_dependencies {
			println!(
				"Optional maybe incomplete std feature found for `{}` at `{}`",
				alias, path
			);
		}

		if !incomplete_dependencies.is_empty() {
			for (alias, path) in incomplete_dependencies {
				println!("Incomplete std feature found for `{}` at `{}`", alias, path);
			}

			process::exit(-1);
		}

		Ok(())
	}
}
