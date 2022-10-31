// std
use std::path::PathBuf;
// crates.io
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct JsonOutput {
	/// Enable JSON output.
	#[arg(long)]
	pub(crate) json_output: bool,
}

#[derive(Debug, Args)]
pub(crate) struct TwoState {
	/// State a's path.
	#[arg(required = true, value_name = "PATH")]
	pub(crate) a: PathBuf,
	/// State b's path.
	#[arg(required = true, value_name = "PATH")]
	pub(crate) b: PathBuf,
}
