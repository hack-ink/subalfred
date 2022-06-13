// crates.io
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSONRPC Id.
pub type Id = usize;

/// Generic JSONRPC request.
#[allow(missing_docs)]
#[derive(Debug, Serialize)]
pub struct Request {
	pub jsonrpc: String,
	pub id: Id,
	pub method: String,
	pub params: Value,
}

/// Generic JSONRPC response.
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct Response<T> {
	pub jsonrpc: String,
	pub id: Id,
	pub result: T,
}
