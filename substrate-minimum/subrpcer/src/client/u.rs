// crates.io
use serde_json::Value;
use ureq::{Error, Response};

pub fn send_rpc(uri: &str, body: &Value) -> Result<Response, Error> {
	ureq::post(uri)
		.set("Content-Type", "application/json;charset=utf-8")
		// TODO: accept reference
		.send_json(body.to_owned())
}
