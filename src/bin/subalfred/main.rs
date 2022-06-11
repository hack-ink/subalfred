#![feature(concat_idents)]
#![warn(missing_docs)]

//! TODO: doc

/// Useful tools set for development.
mod prelude {
	pub use anyhow::Result as AnyResult;
}
use prelude::AnyResult;

mod cli;
use cli::Cli;

mod command;

#[async_std::main]
async fn main() -> AnyResult<()> {
	tracing_subscriber::fmt::init();

	Cli::new().run()
}
