#![feature(concat_idents)]
#![warn(missing_docs)]

//! Subalfred CLI start entry point.

mod prelude {
	pub use ::std::result::Result as StdResult;

	pub use anyhow::Result;

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

mod cli;
mod command;

// #[tokio::main]
fn main() -> prelude::Result<()> {
	cli::Cli::new().run()
}
