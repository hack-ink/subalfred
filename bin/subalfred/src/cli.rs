// std
use std::env;
// crates.io
use clap::{Args, Parser};
// subalfred
use crate::{command::Cmd, prelude::*};

#[derive(Debug, Parser)]
#[command(
	version = concat!(
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("VERGEN_GIT_SHA"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	about,
	rename_all = "kebab",
)]
pub(crate) struct Cli {
	#[command(subcommand)]
	subcmd: Cmd,
	#[command(flatten)]
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
	/// This flag also works with the `RUST_LOG` environment variable.
	/// If you are using `RUST_LOG` at the same time, it will append the value of `RUST_LOG` after the log.
	#[arg(verbatim_doc_comment, global = true, long, short, value_name = "TARGET=LEVEL,*", default_value = "info")]
	log: String,
}
