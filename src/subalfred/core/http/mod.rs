#[cfg(test)] mod test;

// std
use std::sync::Arc;
// crates.io
use once_cell::sync::Lazy;
use reqwest::{Client, Response};
use serde_json::Value;
// hack-ink
use crate::core::{error, Result};

/// Global HTTP client.
pub static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));

/// Send the JSONRPC through the [`CLIENT`] with the given JSON.
pub async fn send_jsonrpc(uri: &str, json: &Value) -> Result<Response> {
	Ok(CLIENT.post(uri).json(json).send().await.map_err(error::Generic::Reqwest)?)
}
