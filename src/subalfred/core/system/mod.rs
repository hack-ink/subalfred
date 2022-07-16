//! The core library about how Subalfred interacts with OS/System.

#[cfg(test)] mod test;

// std
use std::{
	fs::{self, File},
	io::{Read, Write},
	net::TcpListener,
	path::{Path, PathBuf},
};
// hack-ink
use crate::core::{error, Result};

/// System port.
pub type Port = u16;

const E_CALC_SWAP_PATH_FAILED: &str = "[core::system] failed to calculate the swap file path";
const E_NO_AVAILABLE_PORT_FOUND: &str = "[core::system] failed to find an available port";

/// Read the file's content to [`String`].
pub fn read_file_to_string<P>(path: P) -> Result<String>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path).map_err(error::Generic::Io)?;
	let mut content = String::new();

	file.read_to_string(&mut content).map_err(error::Generic::Io)?;

	Ok(content)
}

/// Read the file's content to [`Vec<u8>`].
pub fn read_file_to_vec<P>(path: P) -> Result<Vec<u8>>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path).map_err(error::Generic::Io)?;
	let mut content = Vec::new();

	file.read_to_end(&mut content).map_err(error::Generic::Io)?;

	Ok(content)
}

/// Write the data to the given path file.
pub fn write_data_to_file<P>(path: P, data: &[u8]) -> Result<()>
where
	P: AsRef<Path>,
{
	let mut file = File::create(path).map_err(error::Generic::Io)?;

	Ok(file.write_all(data).map_err(error::Generic::Io)?)
}

/// Swap the file's data with the given one.
///
/// This function will create a temporary file first. Then perform the file-swapping.
pub fn swap_file_data<P>(path: P, data: &[u8]) -> Result<()>
where
	P: AsRef<Path>,
{
	let path = path.as_ref();
	let swapped_path =
		swapped_file_path(path).ok_or_else(|| error::almost_impossible(E_CALC_SWAP_PATH_FAILED))?;

	write_data_to_file(&swapped_path, data)?;
	fs::rename(swapped_path, path).map_err(error::Generic::Io)?;

	Ok(())
}
fn swapped_file_path<P>(path: P) -> Option<PathBuf>
where
	P: AsRef<Path>,
{
	let path = path.as_ref();
	let file_name = path.file_name()?.to_string_lossy();

	Some(path.with_file_name(format!(".{file_name}.swp")))
}

/// Search a available port.
///
/// Skip the system ports, starting from 1001.
pub fn random_available_port() -> Result<Port> {
	for port in 1025..Port::MAX {
		if TcpListener::bind(("127.0.0.1", port)).is_ok() {
			return Ok(port);
		}
	}

	Err(error::almost_impossible(E_NO_AVAILABLE_PORT_FOUND))?
}
