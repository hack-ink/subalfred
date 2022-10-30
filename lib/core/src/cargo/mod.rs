//! Subalfred core Cargo library.

#[cfg(test)] mod test;

mod util;

// std
use std::borrow::Cow;
// crates.io
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand, Node, Package, PackageId, Resolve};
use futures::stream::{self, StreamExt};
use regex::Captures;
// hack-ink
use crate::{prelude::*, system};

/// Get the item from the given source by id.
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
		Ok(self.packages.iter().find(|pkg| &pkg.id == id).ok_or(error::Cargo::GetPackageFailed)?)
	}
}
impl<'a> GetById<'a> for &'a [Node] {
	type Id = PackageId;
	type Item = Node;

	fn get_by_id<'b>(self, id: &'b Self::Id) -> Result<&'a Self::Item>
	where
		'a: 'b,
	{
		Ok(self.iter().find(|node| &node.id == id).ok_or(error::Cargo::GetNodeFailed)?)
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

// TODO: might be useless
// /// Read the [`Manifest`] from the given path.
// pub fn manifest<P>(path: P) -> Result<Manifest>
// where
// 	P: AsRef<Path>,
// {
// 	Ok(Manifest::from_path(path).map_err(error::Cargo::OpenManifestFailed)?)
// }

// TODO: optimize the algorithm
/// Update all the workspace members' versions with the given one.
///
/// If a workspace member depends on another one, the dependency will also be updated.
pub async fn update_member_versions(manifest_path: &str, to: &str) -> Result<()> {
	let metadata = metadata(manifest_path)?;
	let members = if let Some(members) = members(&metadata) {
		members
	} else {
		return Ok(());
	};
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
				util::find_member_dep_regex(&members_deps).replace_all(
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
