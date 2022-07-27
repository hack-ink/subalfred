// std
use std::fmt::Write;
// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;

/// Convert the hex to bytes.
#[derive(Debug, Args)]
pub(crate) struct Hex2BytesCmd {
	/// Hex input.
	///
	/// Example: `0x00000000`.
	#[clap(required = true, value_name = "HEX")]
	hex: String,
	/// Byte string literal style.
	#[clap(long, takes_value = false)]
	byte_string_literal: bool,
}
impl Hex2BytesCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let Self { hex, byte_string_literal } = self;
		let quick_err = || quick_err("invalid byte string");
		let bytes = array_bytes::hex2bytes(hex).map_err(|_| quick_err())?;

		if *byte_string_literal {
			println!("{}", try_as_byte_string_literal(&bytes).ok_or(quick_err())?);
		} else {
			println!("{bytes:?}");
		}

		Ok(())
	}
}

fn try_as_byte_string_literal(bytes: &[u8]) -> Option<String> {
	let mut byte_string_literal = String::new();

	for &byte in bytes {
		match byte {
			40..=126 => byte_string_literal.push(std::char::from_u32(byte as u32).unwrap()),
			_ => write!(byte_string_literal, "\\x{byte:02X}").ok()?,
		}
	}

	Some(byte_string_literal)
}
