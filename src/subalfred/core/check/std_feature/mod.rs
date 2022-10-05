#[cfg(test)] mod test;

// hack-ink
use crate::core::{cargo, prelude::*};

/// Check if the crates' `std` features are enabled correctly.
pub fn check(manifest_path: &str) -> Result<Vec<(String, String)>> {
	let metadata = cargo::metadata(manifest_path)?;
	let members = if let Some(members) = cargo::members(&metadata) {
		members
	} else {
		return Ok(Vec::new());
	};
	let mut disabled_std_deps = Vec::new();

	for pkg in members {
		let path = &pkg.manifest_path;
		let manifest = cargo::manifest(path)?;

		if let Some(std) = manifest.features.get("std") {
			manifest.dependencies.iter().for_each(|(alias, dep)| {
				if let Some(detail) = dep.detail() {
					if !detail.default_features && !std.iter().any(|s| s.contains(alias)) {
						let dep = (alias.clone(), path.to_string());

						disabled_std_deps.push(dep);
					}
				}
			});
		}
	}

	Ok(disabled_std_deps)
}
