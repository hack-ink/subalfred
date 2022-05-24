// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::check::default_features;

/// Check if the crates' `std` features are enabled correctly.
#[derive(Debug, Args)]
pub struct DefaultFeaturesCmd {
	/// Path to `Cargo.toml`.
	#[clap(long, value_name = "PATH", default_value = "./Cargo.toml")]
	manifest_path: String,
}
impl DefaultFeaturesCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { manifest_path } = self;
		let disabled_std_deps = default_features::check(manifest_path)?;

		disabled_std_deps
			.into_iter()
			.for_each(|(alias, path)| println!("`{alias}`'s std feature was disabled in `{path}`"));

		Ok(())
	}
}
