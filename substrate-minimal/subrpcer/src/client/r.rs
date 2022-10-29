// std
use std::sync::Arc;
// crates.io
use once_cell::sync::Lazy;
use reqwest::{Client, Error, Response};
use serde_json::Value;

static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));

/// A simple HTTP post helper which implements with [reqwest](https://crates.io/crates/reqwest).
pub async fn send_jsonrpc(uri: &str, json: &Value) -> Result<Response, Error> {
	CLIENT.post(uri).json(json).send().await
}
