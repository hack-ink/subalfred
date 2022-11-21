// std
use std::{borrow::Cow, path::PathBuf};
// crates.io
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct JsonOutput {
	/// Enable JSON output.
	#[arg(long)]
	pub(crate) json_output: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ManifestPath {
	/// Root `Cargo.toml`'s path.
	///
	/// If `Cargo.toml` wasn't given, Subalfred will search it under the given path.
	#[arg(long, value_name = "PATH", default_value = "./Cargo.toml")]
	pub(crate) manifest_path: PathBuf,
}
impl ManifestPath {
	pub(crate) fn build_path(path: &PathBuf) -> Cow<PathBuf> {
		if path.is_file() {
			Cow::Borrowed(path)
		} else {
			let mut path = path.to_owned();

			path.push("Cargo.toml");

			Cow::Owned(path)
		}
	}

	pub(crate) fn path(&self) -> Cow<PathBuf> {
		Self::build_path(&self.manifest_path)
	}
}

#[derive(Debug, Args)]
pub(crate) struct TwoChainSpec {
	/// Chain spec a's path.
	#[arg(required = true, value_name = "PATH")]
	pub(crate) a: PathBuf,
	/// Chain spec b's path.
	#[arg(required = true, value_name = "PATH")]
	pub(crate) b: PathBuf,
}
