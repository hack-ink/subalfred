// crates.io
use clap::Args;
// subalfred
use crate::{command::shared::ManifestPath, prelude::*};
use subalfred_core::cargo;

/// Update the workspace member versions.
#[derive(Debug, Args)]
pub(crate) struct UpdateCmd {
	/// Target version.
	#[arg(value_name = "VERSION")]
	version: String,
	#[command(flatten)]
	manifest_path: ManifestPath,
}
impl UpdateCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { version, manifest_path } = self;

		cargo::update_member_versions(version, &manifest_path.path().to_string_lossy()).await?;

		Ok(())
	}
}
