//! The core library about how Subalfred interacts with Cargo.

#[cfg(test)] mod test;

mod util;

// std
use std::{borrow::Cow, path::Path};
// crates.io
use cargo_metadata::{Metadata, MetadataCommand, Package};
use cargo_toml::Manifest;
use futures::stream::{self, StreamExt};
use regex::Captures;
// hack-ink
use crate::core::{error, system, Result};

const E_PKG_NOT_FOUND: &str = "[core::cargo] package not found";
const E_BUILD_REGEX_FAILED: &str = "[core::cargo] failed to build the `Regex`";

/// Get the `cargo metadata` result.
pub fn metadata(manifest_path: &str) -> Result<Metadata> {
	Ok(MetadataCommand::new()
		.manifest_path(manifest_path)
		.exec()
		.map_err(error::Cargo::ExecMetadataFailed)?)
}

/// Get all the workspace members from the workspace metadata.
pub fn members(metadata: &Metadata) -> Result<Vec<&Package>> {
	metadata
		.workspace_members
		.iter()
		.map(|id| {
			Ok(util::find_package(metadata, id)
				.ok_or(error::Generic::AlmostImpossible(E_PKG_NOT_FOUND))?)
		})
		.collect::<Result<_>>()
}

/// Read the [`Manifest`] from the given path.
pub fn manifest<P>(path: P) -> Result<Manifest>
where
	P: AsRef<Path>,
{
	Ok(Manifest::from_path(path).map_err(error::Cargo::OpenManifestFailed)?)
}

// TODO: optimize the algorithm
/// Update all the workspace members' versions with the given one.
///
/// If a workspace member depends on another one, the dependency will also be updated.
pub async fn update_members_versions(manifest_path: &str, to: &str) -> Result<()> {
	let metadata = metadata(manifest_path)?;
	let members = members(&metadata)?;
	let mut tasks = stream::iter(&members)
		.map(|pkg| async {
			let members_deps = pkg
				.dependencies
				.iter()
				.filter(|dep| members.iter().any(|pkg| dep.name == pkg.name))
				.collect::<Vec<_>>();
			let content = system::read_file_to_string(&pkg.manifest_path)?;
			let content = content.replacen(&pkg.version.to_string(), to, 1);
			let content = if members_deps.is_empty() {
				Cow::Owned(content)
			} else {
				util::find_member_dep_regex(&members_deps)?.replace_all(
					&content,
					|caps: &Captures| {
						format!("{}\"{}\"", &caps[1], util::align_version(&caps[3], to))
					},
				)
			};

			system::swap_file_data(&pkg.manifest_path, content.as_bytes())
		})
		// Process 64 files concurrently.
		.buffer_unordered(64);

	while let Some(r) = tasks.next().await {
		r?;
	}

	Ok(())
}
