//! Minimal HTTP JSONRPC client implementation.

#[cfg(test)] mod test;

// std
use std::time::Duration;
// crates.io
use serde::{de::DeserializeOwned, Serialize};
// subalfred
use super::*;
use crate::{http::CLIENT, prelude::*};

/// Send a JSONRPC request through the [`CLIENT`].
pub async fn send<S, D>(uri: &str, s: &S, timeout: Duration) -> Result<Response<D>>
where
	S: Serialize,
	D: DeserializeOwned,
{
	Ok(CLIENT
		.post(uri)
		.json(s)
		.timeout(timeout)
		.send()
		.await
		.map_err(error::Generic::Reqwest)?
		.json::<Response<D>>()
		.await
		.map_err(error::Generic::Reqwest)?)
}
