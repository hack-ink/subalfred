pub mod author;
pub mod chain;
pub mod state;

#[cfg(feature = "sender")]
pub mod sender {
	// --- crates.io ---
	use anyhow::Result as AnyResult;
	use isahc::{
		http::{
			header::CONTENT_TYPE, request::Builder as RequestBuilder, Method as HttpMethod,
			Response,
		},
		Body as IsahcBody,
	};
	use serde_json::Value;
	use tracing::trace;

	pub type IsahcResponse = Response<IsahcBody>;

	pub async fn send_rpc(uri: impl AsRef<str>, body: Value) -> AnyResult<IsahcResponse> {
		let mut request_builder = RequestBuilder::new()
			.method(HttpMethod::POST)
			.uri(uri.as_ref());

		request_builder.headers_mut().unwrap().append(
			CONTENT_TYPE,
			"application/json;charset=utf-8".parse().unwrap(),
		);

		let request = request_builder.body(serde_json::to_vec(&body)?).unwrap();
		let result = isahc::send_async(request).await?;

		trace!("{:#?}", result);

		Ok(result)
	}
}

// --- subrpcer ---
#[cfg(feature = "sender")]
pub use sender::*;

// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};

pub fn rpc(method: impl Serialize, params: impl Serialize, id: impl Serialize) -> Value {
	json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}
