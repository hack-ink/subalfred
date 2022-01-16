// --- crates.io ---
use clap::clap;
// --- hack-ink ---
use crate::{cli::Run, AnyResult};

#[derive(Debug, Parser)]
pub struct HashCmd {
	#[clap(required = true, takes_value = true, value_name = "VALUE")]
	data: String,
}
impl Run for HashCmd {
	fn run(&self) -> AnyResult<()> {
		Ok(())
	}
}
