// std
use std::{borrow::Cow, path::PathBuf, process};
// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred_core::check::features;

/// Check if the crates' features are enabled correctly.
#[derive(Debug, Args)]
pub(crate) struct FeaturesCmd {
	/// Path to the target's `Cargo.toml`
	/// The target could be a pallet or runtime.
	///
	/// If `Cargo.toml` wasn't given, Subalfred will search it under the given path.
	#[arg(long, value_name = "PATH", default_value = "./Cargo.toml")]
	manifest_path: PathBuf,
}
impl FeaturesCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { manifest_path } = self;
		let manifest_path = if manifest_path.is_file() {
			Cow::Borrowed(manifest_path)
		} else {
			let mut manifest_path = manifest_path.to_owned();

			manifest_path.push("Cargo.toml");

			Cow::Owned(manifest_path)
		};
		let manifest_path = manifest_path.to_string_lossy();

		println!("checking: {manifest_path}");

		let mut check_passed = true;

		features::check(&manifest_path)?.into_iter().for_each(|(feature, omitteds)| {
			omitteds.into_iter().for_each(|omitted| {
				check_passed = false;

				println!("`{feature}` of `{omitted}` was omitted")
			});
		});

		if check_passed {
			Ok(())
		} else {
			process::exit(-1)
		}
	}
}
