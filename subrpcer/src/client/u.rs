pub use ureq::{self, Error as UreqError};

// --- crates.io ---
use serde_json::Value;
// --- subrpcer ---
use crate::client::Error;

pub type UreqResponse = ureq::Response;

pub fn send_rpc(uri: impl AsRef<str>, body: impl AsRef<Value>) -> Result<UreqResponse, Error> {
	let result = ureq::post(uri.as_ref())
		.set("Content-Type", "application/json;charset=utf-8")
		// TODO: accept reference
		.send_json(body.as_ref().to_owned())?;

	tracing::trace!("{:#?}", result);

	Ok(result)
}
