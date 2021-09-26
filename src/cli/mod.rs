mod args;
use args::*;

mod subcommand;
use subcommand::*;

// --- std ---
use std::env;
// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::AnyResult;

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
	let Opt { args, subcommand } = Opt::from_args();

	env::set_var(
		"RUST_LOG",
		if let Ok(rust_log) = env::var("RUST_LOG") {
			[rust_log, args.log].join(",")
		} else {
			args.log
		},
	);

	// let subalfred = Subalfred::init();

	subcommand.run()
}
