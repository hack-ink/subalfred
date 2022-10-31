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
	/// Path to the state a.
	#[arg(required = true, value_name = "PATH")]
	pub(crate) a: PathBuf,
	/// Path to the second state b.
	#[arg(required = true, value_name = "PATH")]
	pub(crate) b: PathBuf,
}
