// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::check::std_feature;

/// Check if the crates' `std` features are enabled correctly.
#[derive(Debug, Args)]
pub(crate) struct StdFeatureCmd {
	/// Path to the root `Cargo.toml`.
	#[clap(long, value_name = "PATH", default_value = "./Cargo.toml")]
	manifest_path: String,
}
impl StdFeatureCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { manifest_path } = self;
		let disabled_std_deps = std_feature::check(manifest_path)?;

		disabled_std_deps
			.into_iter()
			.for_each(|(alias, path)| println!("`{alias}`'s std feature was disabled in `{path}`"));

		// TODO: exit status

		Ok(())
	}
}
