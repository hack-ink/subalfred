// --- crates.io ---
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
	#[structopt(
		long,
		takes_value = true,
		value_name = "TARGET=LEVEL,*",
		default_value = "",
		global = true
	)]
	pub log: String
	,
}
