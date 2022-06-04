#[cfg(test)] mod test;

// std
use std::{
	borrow::Cow,
	fs::{self, File},
	io::{Read, Write},
};
// crates.io
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{Metadata, MetadataCommand, Package, PackageId};
use cargo_toml::Manifest;
use futures::future;
use regex::{Captures, Regex};
// hack-ink
use crate::core::{error, Result};

const E_BUILD_REGEX_FAILED: &str = "[core::cargo] failed to build the `Regex`";
const E_CALC_SWAP_PATH_FAILED: &str = "[core::cargo] failed to calculate the swap file path";
const E_PKG_NOT_FOUND: &str = "[core::cargo] package not found";

#[derive(Debug)]
enum VersionSpec {
	Majored = 0,
	Minored,
	Patched,
}
impl From<&str> for VersionSpec {
	fn from(s: &str) -> Self {
		match s.chars().fold(0_u8, |acc, c| if c == '.' { acc + 1 } else { acc }) {
			0 => Self::Majored,
			1 => Self::Minored,
			2 => Self::Patched,
			_ => Self::Patched,
		}
	}
}

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
			Ok(find_package(metadata, id)
				.ok_or(error::Generic::AlmostImpossible(E_PKG_NOT_FOUND))?)
		})
		.collect::<Result<_>>()
}

/// Read the [`Manifest`] from the given path.
pub fn manifest(path: impl AsRef<Utf8Path>) -> Result<Manifest> {
	Ok(Manifest::from_path(path.as_ref()).map_err(error::Cargo::OpenManifestFailed)?)
}

// TODO: optimize the algorithm
pub async fn update_members_version(manifest_path: &str, to: &str) -> Result<()> {
	let metadata = metadata(manifest_path)?;
	let members = members(&metadata)?;

	// If the futures are too large, switch to `FuturesUnordered`.
	// TODO: handling result
	let _ = future::join_all(members.iter().map(|pkg| async {
		// Move the ownership here.
		let swapped_path = swap_file_path(&pkg.manifest_path)
			.ok_or(error::Generic::AlmostImpossible(E_CALC_SWAP_PATH_FAILED))?;
		let member_deps = pkg
			.dependencies
			.iter()
			.filter_map(|dep| {
				if members.iter().any(|pkg| dep.name == pkg.name) {
					Some(dep)
				} else {
					None
				}
			})
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
				format!("{}\"{}\"", &captures[1], align_version(&captures[3], to))
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
	// let metadata = metadata(manifest_path)?;
	// let manifests_paths = members(&metadata);
	// let regex = Regex::new(&format!("\"{from}\""))
	// 	.map_err(|_| error::Generic::AlmostImpossible(E_BUILD_REGEX_FAILED))?;

	// // If the futures are too large, switch to `FuturesUnordered`.
	// // TODO: handling result
	// let _ = future::join_all(manifests_paths.into_iter().map(|manifest_path| async {
	// 	// TODO: hidden file or not
	// 	let swap_path = format!("{}.swp", &manifest_path);

	// 	// Read.
	// 	let content = {
	// 		let mut file = File::open(&manifest_path)
	// 			// .map_err(|e| error::Generic::OpenFile { file: manifest_path.clone(), source: e
	// 			// })?;
	// 			.map_err(error::Generic::Io)?;
	// 		let mut content = String::new();

	// 		file.read_to_string(&mut content).map_err(error::Generic::Io)?;

	// 		content
	// 	};
	// 	let content = regex.replace_all(&content, format!("\"{to}\""));

	// 	// Write.
	// 	{
	// 		let mut file = File::create(&swap_path).map_err(error::Generic::Io)?;
	// 		file.write_all(content.as_bytes()).map_err(error::Generic::Io)?;
	// 	}

	// 	// Replace.
	// 	fs::rename(swap_path, manifest_path).map_err(error::Generic::Io)?;

	// 	Result::Ok(())
	// }))
	// .await;

	Ok(())
}

// TODO: move to util
fn find_package<'a>(metadata: &'a Metadata, id: &PackageId) -> Option<&'a Package> {
	metadata.packages.iter().find(|pkg| &pkg.id == id)
}

fn swap_file_path(path: &Utf8PathBuf) -> Option<Utf8PathBuf> {
	let file_name = path.file_name()?;

	Some(path.with_file_name(format!(".{file_name}.swp")))
}

fn align_version<'a>(from: &str, to: &'a str) -> Cow<'a, str> {
	let from_spec = VersionSpec::from(from);
	let to_spec = VersionSpec::from(to);

	match from_spec as i8 - to_spec as i8 {
		-2 => Cow::Owned(to.split_once('.').unwrap_or_default().0.into()),
		-1 => Cow::Owned(to.rsplit_once('.').unwrap_or_default().0.into()),
		0 => Cow::Borrowed(to),
		1 => Cow::Owned(format!("{to}.0")),
		2 => Cow::Owned(format!("{to}.0.0")),
		_ => Cow::Borrowed(to),
	}
}
