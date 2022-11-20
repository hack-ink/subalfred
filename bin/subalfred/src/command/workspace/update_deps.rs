// crates.io
use clap::Args;
// hack-ink
use crate::{command::shared::ManifestPath, prelude::*};
use subalfred_core::cargo;

/// Update the workspace dependency versions.
///
/// To use this command, you must make sure your dependencies were anchored at a branch.
/// This is a general rule of the Polkadot ecosystem.
///
/// We use the regex pattern matching here.
/// So, `git` field must be set before the `branch` field.
///
/// It might look like this:
/// ```toml
/// frame-system = { git = "https://github.com/paritytech/substrate", branch = "polkadot-v.0.0.0" }
/// ```
#[derive(Debug, Args)]
#[command(verbatim_doc_comment)]
pub(crate) struct UpdateDepsCmd {
	/// Target version.
	///
	/// e.g. `0.0.0` will generate `release-v0.0.0` and `polkadot-v0.0.0`
	#[arg(required = true, value_name = "VERSION")]
	version: String,
	#[command(flatten)]
	manifest_path: ManifestPath,
	/// Targets.
	///
	/// e.g. cumulus,polkadot,substrate
	#[arg(long, required = true, value_delimiter = ',', value_name = "REPOSITORY,*")]
	targets: Vec<String>,
}
impl UpdateDepsCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { version, targets, manifest_path } = self;

		cargo::update_dependency_versions(
			version,
			&manifest_path.manifest_path().to_string_lossy(),
			&targets.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
		)
		.await?;

		Ok(())
	}
}
