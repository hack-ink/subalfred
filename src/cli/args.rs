// --- crates.io ---
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
	#[structopt(long, use_delimiter = true, global = true)]
	pub log: Option<Vec<String>>,
}
