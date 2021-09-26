// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::{cli::Run, AnyResult};

#[derive(Debug, StructOpt)]
pub struct HashCmd {
	#[structopt(required = true, takes_value = true, value_name = "VALUE")]
	data: String,
}
impl Run for HashCmd {
	fn run(&self) -> AnyResult<()> {
		Ok(())
	}
}
