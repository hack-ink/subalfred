#[cfg(test)] mod test;

// crates.io
use cargo_metadata::{Metadata, Node};
// hack-ink
use crate::{
	cargo::{self, GetById},
	prelude::*,
};

const E_INVALID_PACKAGE_ID_FORMAT: &str =
	"[core::cargo] invalid package id format, maybe the `cargo-metadata` SPEC changed";

/// Check if the crates' features are enabled correctly.
pub fn check(manifest_path: &str) -> Result<Vec<(String, Vec<String>)>> {
	let metadata = cargo::metadata(manifest_path)?;
	let resolve = cargo::resolve(&metadata)?;
	let root_pkg = cargo::root_package(&metadata)?;
	let root_node = resolve.nodes.get_by_id(&root_pkg.id)?;
	let renamed_pkgs = root_pkg
		.dependencies
		.iter()
		.filter_map(|dep| dep.rename.as_ref().map(|rename| (dep.name.as_str(), rename.as_str())))
		.collect::<Vec<_>>();
	let mut problem_pkgs = Vec::new();

	for (feature, enabled_features) in &root_pkg.features {
		match feature.as_str() {
			// TODO:
			// I think there is a Rust bug here, it should be the `&'static str` actually.
			// Return the `String` to bypass this question temporarily.
			feature @ "std" | feature @ "runtime-benchmarks" | feature @ "try-runtime" =>
				problem_pkgs.push((
					feature.to_owned(),
					check_feature(feature, enabled_features, &metadata, root_node, &renamed_pkgs)?,
				)),
			_ => continue,
		}
	}

	Ok(omitteds)
}

fn check_feature(
	feature: &str,
	enabled_features: &[String],
	metadata: &Metadata,
	root_node: &Node,
) -> Result<Vec<String>> {
	subalfred_util::execution_timer!(format!("check {feature}"));

	let mut omitteds = Vec::new();

	for dep in &root_node.deps {
		let pkg_id = &dep.pkg;
		let pkg_name = pkg_id
			.repr
			// "id": "pallet-a 0.0.0 (path+file:///subalfred/lib/core/src/check/features/mock-runtime/pallet/a)",
			// "pallet-a"
			.split_once(' ')
			.ok_or_else(|| error::almost_impossible(E_INVALID_PACKAGE_ID_FORMAT))?
			.0;
		let pkg_name = renamed_pkgs
			.iter()
			.find_map(|&(name, rename)| if name == pkg_name { Some(rename) } else { None })
			.unwrap_or(pkg_name);
		let pkg = metadata.get_by_id(pkg_id)?;

		// If the dependency has the feature.
		if pkg.features.iter().any(|(feature_, _)| feature_ == feature) {
			let mut omitted = true;

			for enabled_feature in enabled_features {
				if enabled_feature.contains(pkg_name) {
					omitted = false;

					break;
				}
			}

			if omitted {
				omitteds.push(pkg_name.to_owned());
			}
		}
	}

	Ok(omitteds)
}
