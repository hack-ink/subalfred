//! Subalfred core Cargo library.

#[cfg(test)] mod test;
#[cfg(test)] mod test_data;

mod util;

// std
use std::borrow::Cow;
// crates.io
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand, Node, Package, PackageId, Resolve};
use futures::stream::{self, StreamExt};
use regex::Captures;
// hack-ink
use crate::{prelude::*, system};

/// Provide a function to get the item by id from specific source.
pub trait GetById<'a> {
	/// Id type.
	type Id;
	/// Item type.
	type Item;

	/// Get the item by id.
	fn get_by_id<'b>(self, id: &'b Self::Id) -> Result<&'a Self::Item>
	where
		'a: 'b;
}
impl<'a> GetById<'a> for &'a Metadata {
	type Id = PackageId;
	type Item = Package;

	fn get_by_id<'b>(self, id: &'b Self::Id) -> Result<&'a Self::Item>
	where
		'a: 'b,
	{
		Ok(self.packages.iter().find(|p| &p.id == id).ok_or(error::Cargo::GetPackageFailed)?)
	}
}
impl<'a> GetById<'a> for &'a [Node] {
	type Id = PackageId;
	type Item = Node;

	fn get_by_id<'b>(self, id: &'b Self::Id) -> Result<&'a Self::Item>
	where
		'a: 'b,
	{
		Ok(self.iter().find(|n| &n.id == id).ok_or(error::Cargo::GetNodeFailed)?)
	}
}

/// Get the `cargo metadata` result.
pub fn metadata(manifest_path: &str) -> Result<Metadata> {
	Ok(MetadataCommand::new()
		.manifest_path(manifest_path)
		.features(CargoOpt::AllFeatures)
		.exec()
		.map_err(error::Cargo::ExecMetadataFailed)?)
}

/// Get the metadata's root package.
pub fn root_package(metadata: &Metadata) -> Result<&Package> {
	Ok(metadata.root_package().ok_or(error::Cargo::GetRootPackageFailed)?)
}

/// Get the metadata's resolve.
pub fn resolve(metadata: &Metadata) -> Result<&Resolve> {
	Ok(metadata.resolve.as_ref().ok_or(error::Cargo::GetResolveFailed)?)
}

/// Get all the workspace members from the workspace metadata.
pub fn members(metadata: &Metadata) -> Option<Vec<&Package>> {
	metadata.workspace_members.iter().map(|id| util::find_package(metadata, id)).collect()
}

// TODO: optimize the algorithm
/// Update all workspace member versions with the given one.
///
/// If a member depends on other members, this will update them all.
pub async fn update_member_versions(version: &str, manifest_path: &str) -> Result<()> {
	let metadata = metadata(manifest_path)?;
	let Some(members) = members(&metadata) else {
		return Ok(());
	};
	let mut tasks = stream::iter(&members)
		.map(|p| async {
			let members = p
				.dependencies
				.iter()
				.filter(|d| members.iter().any(|p| d.name == p.name))
				.collect::<Vec<_>>();
			let content = system::read_file_to_string(&p.manifest_path)?;
			let content = content.replacen(&p.version.to_string(), version, 1);

			if members.is_empty() {
				system::swap_file_data(&p.manifest_path, content.as_bytes())
			} else {
				replace_member_versions(
					&content,
					&members.iter().map(|m| m.name.as_str()).collect::<Vec<_>>(),
					version,
				);

				system::swap_file_data(&p.manifest_path, content.as_bytes())
			}
		})
		// TODO: configurable
		.buffer_unordered(64);

	while let Some(r) = tasks.next().await {
		r?;
	}

	Ok(())
}
fn replace_member_versions<'a>(
	content: &'a str,
	members: &'a [&'a str],
	version: &'a str,
) -> Cow<'a, str> {
	util::replace_member_versions(members).replace_all(content, |c: &Captures| {
		format!("{}\"{}\"", &c[1], util::align_version(&c[3], version))
	})
}

// TODO: this function isn't general enough, move it to somewhere in the future
// TODO: get rid of `cargo metadata`?
/// Update specific workspace dependencies' version with the given one.
///
/// To use this function, you must make sure your dependencies were anchored at a branch.
/// This is a general rule of the Polkadot ecosystem.
///
/// We use the regex pattern matching here.
/// So, `git` field must be set before the `branch` field.
///
/// It might look like this:
/// ```toml
/// frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v.0.0.0" }
/// ```
pub async fn update_dependency_versions(
	version: &str,
	manifest_path: &str,
	targets: &[&str],
) -> Result<()> {
	let metadata = metadata(manifest_path)?;
	let Some(members) = members(&metadata) else {
		return Ok(());
	};
	let mut tasks = stream::iter(members)
		.map(|p| async {
			let content = system::read_file_to_string(&p.manifest_path)?;
			let new_content = replace_target_versions(&content, targets, version);

			if content == new_content {
				Ok(())
			} else {
				system::swap_file_data(&p.manifest_path, new_content.as_bytes())
			}
		})
		// TODO: configurable
		.buffer_unordered(64);

	while let Some(r) = tasks.next().await {
		r?;
	}

	Ok(())
}
fn replace_target_versions<'a>(
	content: &'a str,
	targets: &'a [&'a str],
	version: &'a str,
) -> Cow<'a, str> {
	util::replace_target_versions(targets).replace_all(content, |c: &Captures| {
		format!(
			"{}\"{}\"",
			&c[1],
			if &c[2] == "polkadot" {
				format!("release-v{version}")
			} else {
				format!("polkadot-v{version}")
			}
		)
	})
}
