// --- crates.io ---
use anyhow::Result;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error("Fail to convert `{}` to bytes", hex_str)]
	InvalidHexLength { hex_str: String },
}

#[macro_export]
macro_rules! array_unchecked {
	($vec:expr, $len:expr) => {{
		unsafe { *($vec.as_ptr() as *const [u8; $len]) }
		}};
}

pub fn bytes(hex_str: &str) -> Result<Vec<u8>> {
	if hex_str.len() % 2 != 0 {
		Err(Error::InvalidHexLength {
			hex_str: hex_str.into(),
		})?;
	}

	let hex_str = hex_str.trim_start_matches("0x");

	Ok((0..hex_str.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).map_err(Into::into))
		.collect::<Result<Vec<_>>>()?)
}

pub fn hex_str(bytes: &[u8]) -> String {
	bytes
		.iter()
		.map(|byte| format!("{:02x}", byte))
		.collect::<Vec<_>>()
		.join("")
}
