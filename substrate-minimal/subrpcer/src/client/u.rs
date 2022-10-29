// crates.io
use serde_json::Value;
use ureq::{Error, Response};

/// A simple HTTP post helper which implements with [ureq](https://crates.io/crates/ureq).
pub fn send_jsonrpc(uri: &str, body: &Value) -> Result<Response, Error> {
	ureq::post(uri)
		.set("Content-Type", "application/json;charset=utf-8")
		// TODO: accept reference
		.send_json(body.to_owned())
}
