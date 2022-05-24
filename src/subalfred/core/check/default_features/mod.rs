#[cfg(test)] mod test;

// hack-ink
use crate::core::{cargo, Result};

/// Check if the crates' `std` features are enabled correctly.
pub fn check(manifest_path: &str) -> Result<Vec<(String, String)>> {
	let metadata = cargo::metadata(manifest_path)?;
	let manifest_paths = cargo::members_manifest_paths(&metadata);
	let mut disabled_std_deps = Vec::new();

	for manifest_path in manifest_paths {
		let manifest = cargo::manifest(&manifest_path)?;

		if let Some(std) = manifest.features.get("std") {
			manifest.dependencies.iter().for_each(|(alias, dep)| {
				if let Some(detail) = dep.detail() {
					if let Some(default_features) = detail.default_features {
						if !default_features && !std.contains(&format!("{}/std", alias)) {
							let dep = (alias.clone(), manifest_path.clone());

							disabled_std_deps.push(dep);
						}
					}
				}
			});
		}
	}

	Ok(disabled_std_deps)
}
