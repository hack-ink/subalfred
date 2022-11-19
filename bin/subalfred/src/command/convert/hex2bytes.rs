// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

/// Convert hex to bytes.
#[derive(Debug, Args)]
pub(crate) struct Hex2BytesCmd {
	/// Hex input.
	///
	/// e.g. `0x00000000`
	#[arg(required = true, value_name = "HEX")]
	hex: String,
}
impl Hex2BytesCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { hex } = self;
		let bytes = array_bytes::hex2bytes(hex).map_err(|_| quick_err("invalid hex input"))?;

		println!("{bytes:?}");

		Ok(())
	}
}
