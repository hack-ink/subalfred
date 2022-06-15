//! Minimal JSONRPC HTTP client implementation.

#[cfg(test)] mod test;

// crates.io
use serde::{de::DeserializeOwned, Serialize};
// hack-ink
use super::*;
use crate::core::{error, http::CLIENT, Result};

/// Send the JSONRPC through the [`CLIENT`] with the given JSON.
pub async fn send<S, D>(uri: &str, s: &S) -> Result<Response<D>>
where
	S: Serialize,
	D: DeserializeOwned,
{
	Ok(CLIENT
		.post(uri)
		.json(s)
		.send()
		.await
		.map_err(error::Generic::Reqwest)?
		.json::<Response<D>>()
		.await
		.map_err(error::Generic::Reqwest)?)
}
