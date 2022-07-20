// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::check::std_feature;

// TODO: check if the dependency has the std feature.
/// Check if the crates' `std` features are enabled correctly.
#[derive(Debug, Args)]
pub struct StdFeatureCmd {
	/// Path to the root `Cargo.toml`.
	#[clap(long, value_name = "PATH", default_value = "./Cargo.toml")]
	manifest_path: String,
}
impl StdFeatureCmd {
	pub fn run(&self) -> Result<()> {
		let Self { manifest_path } = self;
		// TODO: check cargo metadata
		let disabled_std_deps = std_feature::check(manifest_path)?;

		disabled_std_deps
			.into_iter()
			.for_each(|(alias, path)| println!("`{alias}`'s std feature was disabled in `{path}`"));

		// TODO: exit status

		Ok(())
	}
}
