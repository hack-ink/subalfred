// std
use std::{
	fs::{self, File},
	io::{Read, Write},
	path::Path,
};
// crates.io
use camino::{Utf8Path, Utf8PathBuf};
// hack-ink
use crate::core::{error, Result};

const E_CALC_SWAP_PATH_FAILED: &str = "[core::cargo] failed to calculate the swap file path";

pub fn read_file_to_string<P>(path: P) -> Result<String>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path).map_err(error::Generic::Io)?;
	let mut content = String::new();

	file.read_to_string(&mut content).map_err(error::Generic::Io)?;

	Ok(content)
}

pub fn swap_file_data<P>(path: P, data: &[u8]) -> Result<()>
where
	P: AsRef<Utf8Path>,
{
	let path = path.as_ref();
	let swapped_path =
		swapped_file_path(path).ok_or(error::Generic::AlmostImpossible(E_CALC_SWAP_PATH_FAILED))?;
	let mut file = File::create(&swapped_path).map_err(error::Generic::Io)?;

	file.write_all(data).map_err(error::Generic::Io)?;
	fs::rename(swapped_path, path).map_err(error::Generic::Io)?;

	Ok(())
}

fn swapped_file_path<P>(path: P) -> Option<Utf8PathBuf>
where
	P: AsRef<Utf8Path>,
{
	let path = path.as_ref();
	let file_name = path.file_name()?;

	Some(path.with_file_name(format!(".{file_name}.swp")))
}
