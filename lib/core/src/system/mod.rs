//! Subalfred core system library.

#[cfg(test)] mod test;

// std
use std::{
	fs::{self, File},
	io::{Read, Write},
	net::TcpListener,
	path::{Path, PathBuf},
};
// crates.io
use serde::de::DeserializeOwned;
use tokio::{fs::File as FileAsync, io::AsyncReadExt};
// hack-ink
use crate::prelude::*;

/// System port type.
///
/// Basically, it is a [`u16`].
pub type Port = u16;

/// Read the file's content into a [`String`].
pub fn read_file_to_string<P>(path: P) -> Result<String>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path).map_err(error::Generic::Io)?;
	let mut content = String::new();

	file.read_to_string(&mut content).map_err(error::Generic::Io)?;

	Ok(content)
}
/// Async version of [`read_file_to_string`].
pub async fn read_file_to_string_async<P>(path: P) -> Result<String>
where
	P: AsRef<Path>,
{
	let mut file = FileAsync::open(path).await.map_err(error::Generic::Io)?;
	let mut content = String::new();

	file.read_to_string(&mut content).await.map_err(error::Generic::Io)?;

	Ok(content)
}

/// Read the file's content into a [`Vec<u8>`].
pub fn read_file_to_vec<P>(path: P) -> Result<Vec<u8>>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path).map_err(error::Generic::Io)?;
	let mut content = Vec::new();

	file.read_to_end(&mut content).map_err(error::Generic::Io)?;

	Ok(content)
}
/// Async version of [`read_file_to_vec`].
pub async fn read_file_to_vec_async<P>(path: P) -> Result<Vec<u8>>
where
	P: AsRef<Path>,
{
	let mut file = FileAsync::open(path).await.map_err(error::Generic::Io)?;
	let mut content = Vec::new();

	file.read_to_end(&mut content).await.map_err(error::Generic::Io)?;

	Ok(content)
}

/// Read the file's content into a struct implemented [`DeserializeOwned`].
pub fn read_file_to_struct<P, T>(path: P) -> Result<T>
where
	P: AsRef<Path>,
	T: DeserializeOwned,
{
	subalfred_util::execution_timer!("read file to struct");

	let content = {
		subalfred_util::execution_timer!("read json");

		read_file_to_vec(path)?
	};
	let result = {
		subalfred_util::execution_timer!("parse json");

		serde_json::from_slice(&content).map_err(error::Generic::Serde)?
	};

	Ok(result)
}
/// Async version of [`read_file_to_struct`].
pub async fn read_file_to_struct_async<P, T>(path: P) -> Result<T>
where
	P: AsRef<Path>,
	T: DeserializeOwned,
{
	subalfred_util::execution_timer!("read file to struct async");

	let content = {
		subalfred_util::execution_timer!("read json async");

		read_file_to_vec(path)?
	};
	let result = {
		subalfred_util::execution_timer!("parse json async");

		serde_json::from_slice(&content).map_err(error::Generic::Serde)?
	};

	Ok(result)
}

/// Write the data to file at the given path.
///
/// If the file has already existed, then it will be overwritten.
/// Otherwise, this will create a file at the given path.
pub fn write_data_to_file<P>(path: P, data: &[u8]) -> Result<()>
where
	P: AsRef<Path>,
{
	let mut file = File::create(path).map_err(error::Generic::Io)?;

	Ok(file.write_all(data).map_err(error::Generic::Io)?)
}

/// Swap the file's data with the given one.
///
/// This will create a temporary file first, then perform the file-swapping.
pub fn swap_file_data<P>(path: P, data: &[u8]) -> Result<()>
where
	P: AsRef<Path>,
{
	let path = path.as_ref();
	let swapped_path = swapped_file_path(path)
		.ok_or_else(|| error::System::NoFileNameInPath(path.to_path_buf()))?;

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

/// Search an available port.
///
/// Skip the system used ports, starting from 1001.
pub fn random_available_port() -> Result<Port> {
	for port in 1025..Port::MAX {
		if TcpListener::bind(("127.0.0.1", port)).is_ok() {
			return Ok(port);
		}
	}

	Err(error::System::NoAvailablePortFound)?
}
