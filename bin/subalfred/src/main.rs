#![deny(missing_docs)]

//! Subalfred CLI start entry point.

mod prelude {
	pub use std::result::Result as StdResult;

	pub use anyhow::Result;

	pub(crate) use crate::util;

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
mod util;

// #[tokio::main]
fn main() -> prelude::Result<()> {
	color_eyre::install().map_err(|e| anyhow::anyhow!(e))?;
	cli::Cli::new().run()?;

	Ok(())
}
