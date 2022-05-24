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

		env::set_var("RUST_LOG", {
			let log = cli.global_args.log.as_deref().unwrap_or("");

			if let Ok(extra_log) = env::var("RUST_LOG") {
				format!("{log},{extra_log}")
			} else {
				log.into()
			}
		});

		cli
	}

	pub fn run(&self) -> AnyResult<()> {
		self.subcmd.run()
	}
}

#[derive(Debug, Args)]
pub struct GlobalArgs {
	/// Set a custom logging filter. Also, work wit the `RUST_LOG` environment variable.
	#[clap(long, value_name = "TARGET=LEVEL,*", global = true)]
	pub log: Option<String>,
}
