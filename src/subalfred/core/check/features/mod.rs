#[cfg(test)] mod test;

// crates.io
use cargo_metadata::Node;
// hack-ink
use crate::core::{
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
	let nodes = &resolve.nodes;
	let root_node = nodes.get_by_id(&root_pkg.id)?;
	let mut omitteds = Vec::new();

	for (feature, enabled_features) in &root_pkg.features {
		match feature.as_str() {
			// TODO:
			// I think there is a Rust bug here, it should be the `&'static str` actually.
			// Return the `String` to bypass this question temporarily.
			feature @ "std" | feature @ "runtime-benchmarks" | feature @ "try-runtime" => omitteds
				.push((
					feature.to_owned(),
					check_feature(feature, enabled_features, nodes, root_node)?,
				)),
			_ => continue,
		}
	}

	Ok(omitteds)
}

fn check_feature(
	feature: &str,
	enabled_features: &[String],
	nodes: &[Node],
	root_node: &Node,
) -> Result<Vec<String>> {
	crate::execution_timer!(format!("check {feature}"));

	let mut omitteds = Vec::new();

	for dep in &root_node.deps {
		let node = nodes.get_by_id(&dep.pkg)?;
		let pkg_name = node
			.id
			.repr
			.split_once(' ')
			.ok_or_else(|| error::almost_impossible(E_INVALID_PACKAGE_ID_FORMAT))?
			.0;

		// If the dependency has the feature.
		if node.features.iter().any(|feature_| feature_ == feature) {
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
