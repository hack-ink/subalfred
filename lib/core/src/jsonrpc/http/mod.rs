//! Minimal HTTP JSONRPC client implementation.

#[cfg(test)] mod test;

// crates.io
use serde::{de::DeserializeOwned, Serialize};
// hack-ink
use super::*;
use crate::{http::CLIENT, prelude::*};

/// Send a JSONRPC request through the [`CLIENT`].
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
