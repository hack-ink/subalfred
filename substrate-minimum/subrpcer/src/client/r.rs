// std
use std::sync::Arc;
// crates.io
use once_cell::sync::Lazy;
use reqwest::{Client, Error, Response};
use serde_json::Value;

pub static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));

pub async fn send_rpc(uri: &str, json: &Value) -> Result<Response, Error> {
	CLIENT.post(uri).json(json).send().await
}
