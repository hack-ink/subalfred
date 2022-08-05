// std
use std::fmt::Write;
// hack-ink
use crate::prelude::*;

pub(crate) fn vec_literal_string_try_as_vec(s: &str) -> Result<Vec<u8>> {
	if !(s.starts_with('[') && s.ends_with(']')) {
		Err(quick_err("vec literal string must be start with '[' and end with ']'"))?;
	}
	if s.len() < 3 {
		Err(quick_err("empty vec"))?;
	}

	Ok(s[1..s.len() - 1]
		.split(',')
		.map(|s| s.trim_matches(' ').parse::<u8>())
		.collect::<StdResult<Vec<_>, _>>()?)
}

pub(crate) fn vec_try_as_byte_string_literal(bytes: &[u8]) -> Result<String> {
	let mut byte_string_literal = String::new();

	for &byte in bytes {
		match byte {
			40..=126 => byte_string_literal.push(byte as _),
			_ => write!(byte_string_literal, "\\x{byte:02X}")?,
		}
	}

	Ok(byte_string_literal)
}
