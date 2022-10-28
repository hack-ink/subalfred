// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

/// Convert bytes to hex.
#[derive(Debug, Args)]
pub(crate) struct Bytes2HexCmd {
	/// Bytes input.
	///
	/// Example: `[0, 0, 0, 0]`.
	#[arg(required = true, value_name = "BYTES")]
	bytes: String,
}
impl Bytes2HexCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { bytes } = self;

		// May use `clap::value_parser!` here.
		let bytes = util::vec_literal_string_try_as_vec(bytes)?;

		println!("{}", array_bytes::bytes2hex("0x", bytes.as_slice()));

		Ok(())
	}
}
