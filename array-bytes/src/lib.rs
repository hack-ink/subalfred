// --- std ---
use std::{char, num::ParseIntError};
// --- crates.io ---
use thiserror::Error as ThisError;

pub type ArrayBytesResult<T> = Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error("Fail to convert {} to bytes", hex_str)]
	InvalidHexLength { hex_str: String },
	#[error("Fail to parse int")]
	InvalidChar(#[from] ParseIntError),
}

#[macro_export]
macro_rules! bytes_array_unchecked {
	($bytes:expr, $len:expr) => {{
		unsafe { *($bytes.as_ptr() as *const [u8; $len]) }
		}};
}

#[macro_export]
macro_rules! hex_str_array_unchecked {
	($hex_str:expr, $len:expr) => {{
		$crate::bytes_array_unchecked!($crate::bytes_unchecked($hex_str), $len)
		}};
}

pub fn bytes(hex_str: impl AsRef<str>) -> ArrayBytesResult<Vec<u8>> {
	let hex_str = hex_str.as_ref();

	if hex_str.len() % 2 != 0 {
		Err(Error::InvalidHexLength {
			hex_str: hex_str.into(),
		})?;
	}

	Ok(
		(if hex_str.starts_with("0x") { 2 } else { 0 }..hex_str.len())
			.step_by(2)
			.map(|i| Ok(u8::from_str_radix(&hex_str[i..i + 2], 16)?))
			.collect::<ArrayBytesResult<_>>()?,
	)
}
pub fn bytes_unchecked(hex_str: impl AsRef<str>) -> Vec<u8> {
	let hex_str = hex_str.as_ref();

	(if hex_str.starts_with("0x") { 2 } else { 0 }..hex_str.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
		.collect()
}

pub fn hex_str(prefix: impl AsRef<str>, bytes: impl AsRef<[u8]>) -> String {
	let prefix = prefix.as_ref();
	let bytes = bytes.as_ref();
	let mut hex_str = String::with_capacity(prefix.len() + bytes.len() * 2);

	for byte in prefix.chars() {
		hex_str.push(byte);
	}
	for byte in bytes.iter() {
		hex_str.push(char::from_digit((byte >> 4) as _, 16).unwrap());
		hex_str.push(char::from_digit((byte & 0xf) as _, 16).unwrap());
	}

	hex_str
}
