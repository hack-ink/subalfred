#[cfg(test)] mod test;

// std
use std::sync::Arc;
// crates.io
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
// hack-ink
use crate::core::{error, jsonrpc::Response, Result};

/// Global HTTP client.
pub static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));

/// Send the JSONRPC through the [`CLIENT`] with the given JSON.
pub async fn send_jsonrpc<S, D>(uri: &str, json: &S) -> Result<Response<D>>
where
	S: Serialize,
	D: DeserializeOwned,
{
	Ok(CLIENT
		.post(uri)
		.json(json)
		.send()
		.await
		.map_err(error::Generic::Reqwest)?
		.json::<Response<D>>()
		.await
		.map_err(error::Generic::Reqwest)?)
}
