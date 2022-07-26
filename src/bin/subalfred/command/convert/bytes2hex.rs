// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

/// Convert the bytes to hex.
#[derive(Debug, Args)]
pub(crate) struct Bytes2HexCmd {
	/// Bytes input.
	///
	/// Example: `[0, 0, 0, 0]`.
	#[clap(required = true, value_name = "BYTES")]
	bytes: String,
}
impl Bytes2HexCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { bytes } = self;


		// TODO: `clap::value_parser!`
		if !(bytes.starts_with('[') && bytes.ends_with(']')) {
			Err(quick_err("Invalid bytes input."))?;
		}

		let bytes = bytes[1..bytes.len() - 1]
			.split(',')
			.map(|s| s.trim_matches(' ').parse::<u8>())
			.collect::<StdResult<Vec<_>, _>>()?;

		println!("{}", array_bytes::bytes2hex("0x", bytes.as_slice()));

		Ok(())
	}
}
