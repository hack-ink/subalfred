// crates.io
use clap::Args;
// subalfred
use crate::prelude::*;

/// Convert ascii to hex.
#[derive(Debug, Args)]
pub(crate) struct Ascii2HexCmd {
	/// Ascii data input.
	#[arg(value_name = "ASCII")]
	data: String,
}
impl Ascii2HexCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { data } = self;

		println!("{}", array_bytes::bytes2hex("0x", data.as_bytes()));

		Ok(())
	}
}
