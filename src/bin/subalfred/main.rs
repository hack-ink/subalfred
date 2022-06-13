#![feature(concat_idents)]
#![warn(missing_docs)]

//! TODO: doc

/// Useful tools set for development.
mod prelude {
	pub use anyhow::Result as AnyResult;

	// std
	use std::fmt::Debug;
	// crates.io
	use anyhow::Error;

	pub fn debug_err<E>(e: E) -> Error
	where
		E: Debug,
	{
		anyhow::anyhow!("{e:?}")
	}
}
use prelude::AnyResult;

mod cli;
use cli::Cli;

mod command;

fn main() -> AnyResult<()> {
	tracing_subscriber::fmt::init();

	Cli::new().run()
}
