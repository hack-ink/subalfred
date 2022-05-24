#[cfg(test)] mod test;

// crate.io
use cargo_metadata::{Metadata, MetadataCommand, PackageId};
use cargo_toml::Manifest;
// hack-ink
use crate::core::{error, Result};

/// Get the `cargo metadata` result.
pub fn metadata(manifest_path: &str) -> Result<Metadata> {
	Ok(MetadataCommand::new()
		.manifest_path(manifest_path)
		.exec()
		.map_err(|e| error::Cargo::ExecMetadataFailed(e))?)
}

/// Get all the workspace members' manifest paths from the workspace metadata.
pub fn members_manifest_paths(metadata: &Metadata) -> Vec<String> {
	fn manifest_path(package_id: &PackageId) -> String {
		let repr = &package_id.repr;
		let (_l, r) =
			repr.split_once("//").unwrap_or_else(|| panic!("Invalid package id: `{package_id}`"));
		let path = &r[..r.len() - 1];
		let manifest_path = format!("{path}/Cargo.toml");

		manifest_path
	}

	metadata.workspace_members.iter().map(manifest_path).collect()
}

/// Read the [`Manifest`] from the given path.
pub fn manifest(manifest_path: &str) -> Result<Manifest> {
	Ok(Manifest::from_path(manifest_path).map_err(|e| error::Cargo::OpenManifestFailed(e))?)
}
