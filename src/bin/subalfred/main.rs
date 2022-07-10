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

	pub fn quick_err<E>(e: E) -> Error
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

// #[tokio::main]
fn main() -> AnyResult<()> {
	Cli::new().run()
}
