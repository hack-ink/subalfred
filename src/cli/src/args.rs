// crates.io
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
	#[clap(
		long,
		takes_value = true,
		value_name = "TARGET=LEVEL,*",
		default_value = "",
		global = true
	)]
	pub log: String,
}
