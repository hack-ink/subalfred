// std
use std::env;
// crates.io
use clap::{Args, Parser};
// hack-ink
use crate::{command::Cmd, prelude::*};

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
pub struct Cli {
	#[clap(subcommand)]
	subcmd: Cmd,
	#[clap(flatten)]
	global_args: GlobalArgs,
}
impl Cli {
	pub fn new() -> Self {
		let cli = Self::parse();

		if let Ok(extra_log) = env::var("RUST_LOG") {
			env::set_var("RUST_LOG", format!("{},{extra_log}", &cli.global_args.log));
		} else {
			env::set_var("RUST_LOG", &cli.global_args.log);
		}

		tracing_subscriber::fmt::init();

		cli
	}

	pub fn run(&self) -> AnyResult<()> {
		self.subcmd.run()
	}
}

#[derive(Debug, Args)]
pub struct GlobalArgs {
	/// Set a custom logging filter. Also, work with the `RUST_LOG` environment variable.
	#[clap(global = true, long, value_name = "TARGET=LEVEL,*", default_value = "info")]
	pub log: String,
}
