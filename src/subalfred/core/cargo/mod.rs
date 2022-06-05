#[cfg(test)] mod test;

mod util;

// std
use std::{
	borrow::Cow,
	fs::{self, File},
	io::{Read, Write},
	path::Path,
};
// crates.io
use cargo_metadata::{Metadata, MetadataCommand, Package};
use cargo_toml::Manifest;
use futures::future;
use regex::{Captures, Regex};
// hack-ink
use crate::core::{error, system, Result};

const E_BUILD_REGEX_FAILED: &str = "[core::cargo] failed to build the `Regex`";
const E_CALC_SWAP_PATH_FAILED: &str = "[core::cargo] failed to calculate the swap file path";
const E_PKG_NOT_FOUND: &str = "[core::cargo] package not found";

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
pub fn manifest(path: impl AsRef<Path>) -> Result<Manifest> {
	Ok(Manifest::from_path(path).map_err(error::Cargo::OpenManifestFailed)?)
}

// TODO: optimize the algorithm
pub async fn update_members_version(manifest_path: &str, to: &str) -> Result<()> {
	let metadata = metadata(manifest_path)?;
	let members = members(&metadata)?;

	// If the futures are too large, switch to `FuturesUnordered`.
	// TODO: handling result
	let _ = future::join_all(members.iter().map(|pkg| async {
		// Move the ownership here.
		let swapped_path = system::swap_file_path(&pkg.manifest_path)
			.ok_or(error::Generic::AlmostImpossible(E_CALC_SWAP_PATH_FAILED))?;
		let member_deps = pkg
			.dependencies
			.iter()
			.filter(|dep| members.iter().any(|pkg| dep.name == pkg.name))
			.collect::<Vec<_>>();

		// Read.
		let content = {
			let mut file = File::open(&pkg.manifest_path).map_err(error::Generic::Io)?;
			let mut content = String::new();

			file.read_to_string(&mut content).map_err(error::Generic::Io)?;

			content
		};

		// Replace content.
		let content = content.replacen(&pkg.version.to_string(), to, 1);
		let content = if member_deps.is_empty() {
			Cow::Borrowed(content.as_str())
		} else {
			let regex = Regex::new(&format!(
				"(({}) *?= *?\\{{ *?version *?= *?)\"(.+?)\"",
				member_deps
					.iter()
					.map(|dep| dep.name.replace('-', "\\-"))
					.collect::<Vec<_>>()
					.join("|"),
			))
			.map_err(|_| error::Generic::AlmostImpossible(E_BUILD_REGEX_FAILED))?;

			regex.replace_all(&content, |captures: &Captures| {
				format!("{}\"{}\"", &captures[1], util::align_version(&captures[3], to))
			})
		};

		// Write.
		{
			let mut file = File::create(&swapped_path).map_err(error::Generic::Io)?;
			file.write_all(content.as_bytes()).map_err(error::Generic::Io)?;
		}

		// Replace.
		fs::rename(swapped_path, &pkg.manifest_path).map_err(error::Generic::Io)?;

		Result::Ok(())
	}))
	.await;

	Ok(())
}

#[allow(unused)]
pub async fn update_deps_version(manifest_path: &str, from: &str, to: &str) -> Result<()> {
	Ok(())
}
