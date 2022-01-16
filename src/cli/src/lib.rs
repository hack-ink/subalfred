#![feature(concat_idents)]

mod args;
use args::*;

mod subcommand;
use subcommand::*;

// --- std ---
use std::env;
// --- crates.io ---
use anyhow::Result as AnyResult;
use clap::Parser;
// --- hack-ink ---
use executor::Executor;

trait Run {
	fn run(&self) -> AnyResult<()>;
}

#[derive(Debug, Parser)]
#[clap(
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
struct Cli {
	#[clap(subcommand)]
	subcommand: Subcommand,
	#[clap(flatten)]
	args: Args,
}
impl Run for Cli {
	fn run(&self) -> AnyResult<()> {
		self.subcommand.run()
	}
}

pub fn run() -> AnyResult<()> {
	let Cli { args, subcommand } = Cli::parse();

	env::set_var(
		"RUST_LOG",
		if let Ok(rust_log) = env::var("RUST_LOG") {
			[rust_log, args.log].join(",")
		} else {
			args.log
		},
	);

	subcommand.run()
}
