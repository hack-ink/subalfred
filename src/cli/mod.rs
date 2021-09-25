mod args;
use args::*;

mod subcommand;
use subcommand::*;

// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::{AnyResult, Subalfred};

trait Run {
	fn run(&self) -> AnyResult<()>;
}

#[derive(Debug, StructOpt)]
#[structopt(
	version = concat!(
		env!("VERGEN_BUILD_SEMVER"),
		"-",
		env!("VERGEN_GIT_SHA_SHORT"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	author,
	about,
	rename_all = "kebab",
)]
struct Opt {
	#[structopt(subcommand)]
	subcommand: Subcommand,
	#[structopt(flatten)]
	args: Args,
}
impl Run for Opt {
	fn run(&self) -> AnyResult<()> {
		self.subcommand.run()
	}
}

pub fn run() -> AnyResult<()> {
	let opt = Opt::from_args();
	let subalfred = Subalfred::init();

	dbg!(&opt);

	opt.subcommand.run()
}
