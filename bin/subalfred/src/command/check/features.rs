// std
use std::process;
// crates.io
use clap::Args;
// hack-ink
use crate::{command::shared::ManifestPath, prelude::*};
use subalfred_core::check::features;

/// Check if the crates' features are enabled correctly.
#[derive(Debug, Args)]
pub(crate) struct FeaturesCmd {
	#[command(flatten)]
	manifest_path: ManifestPath,
}
impl FeaturesCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { manifest_path } = self;
		let manifest_path = manifest_path.manifest_path();
		let manifest_path = manifest_path.to_string_lossy();

		println!("checking: {manifest_path}");

		let mut check_passed = true;

		features::check(&manifest_path)?.into_iter().for_each(|(feature, problem_pkgs)| {
			problem_pkgs.into_iter().for_each(|problem_pkg| {
				check_passed = false;

				println!("incomplete `{feature}` of `{problem_pkg}`")
			});
		});

		if check_passed {
			Ok(())
		} else {
			process::exit(-1)
		}
	}
}
