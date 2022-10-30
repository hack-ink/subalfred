// std
use std::env;
// crates.io
use clap::{Args, Parser};
// hack-ink
use crate::{command::Cmd, prelude::*};

#[derive(Debug, Parser)]
#[command(
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
pub(crate) struct Cli {
	#[clap(subcommand)]
	subcmd: Cmd,
	#[clap(flatten)]
	global_args: GlobalArgs,
}
impl Cli {
	pub(crate) fn new() -> Self {
		let cli = Self::parse();

		if let Ok(extra_log) = env::var("RUST_LOG") {
			env::set_var("RUST_LOG", format!("{},{extra_log}", &cli.global_args.log));
		} else {
			env::set_var("RUST_LOG", &cli.global_args.log);
		}

		tracing_subscriber::fmt::init();

		cli
	}

	pub(crate) fn run(&self) -> Result<()> {
		self.subcmd.run()
	}
}

#[derive(Debug, Args)]
struct GlobalArgs {
	/// Set a custom log filter.
	///
	/// This flag is also working with the `RUST_LOG` environment variable.
	/// If you use `RUST_LOG` simultaneously, this will append `RUST_LOG`'s value after the log.
	#[arg(global = true, long, short, value_name = "TARGET=LEVEL,*", default_value = "info")]
	log: String,
}
