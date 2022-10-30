// crates.io
use clap::{Args, ValueEnum};
// hack-ink
use crate::prelude::*;
use BytesStringKind::*;

/// Convert bytes between several different styles.
#[derive(Debug, Args)]
pub(crate) struct BytesStyleCmd {
	/// Bytes data input.
	#[arg(required = true, value_name = "BYTES")]
	bytes: String,
	/// Origin style.
	#[arg(value_enum, long, required = true, value_name = "BYTES STYLE")]
	from: BytesStringKind,
	/// Target style.
	#[arg(value_enum, long, required = true, value_name = "BYTES STYLE")]
	to: BytesStringKind,
}
impl BytesStyleCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { bytes, from, to } = self;

		match (from, to) {
			(ByteStringLiteral, ByteStringLiteral) | (VecString, VecString) => {
				println!("{}", bytes);
			},
			(ByteStringLiteral, VecString) => {
				let byte_string_literal = unescaper::unescape(bytes)?;
				let vec = byte_string_literal.as_bytes();

				println!("{vec:?}");
			},
			(VecString, ByteStringLiteral) => {
				let vec = util::vec_literal_string_try_as_vec(bytes)?;
				let byte_string_literal = util::vec_try_as_byte_string_literal(&vec)?;

				println!("{byte_string_literal}");
			},
		}

		Ok(())
	}
}

#[derive(Clone, Debug, ValueEnum)]
enum BytesStringKind {
	ByteStringLiteral,
	VecString,
}
