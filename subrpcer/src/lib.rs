pub mod author;
pub mod chain;
pub mod state;

#[cfg(feature = "sender")]
pub mod sender {
	// --- crates.io ---
	use isahc::{
		http::{
			header::CONTENT_TYPE, request::Builder as RequestBuilder, Method as HttpMethod,
			Response,
		},
		AsyncBody as IsahcBody, Error as IsahcError,
	};
	use serde_json::{Error as RawSerdeJsonError, Value};
	use thiserror::Error as ThisError;
	use tracing::trace;

	pub type SubrpcerResult<T> = Result<T, Error>;
	pub type IsahcResponse = Response<IsahcBody>;

	#[derive(Debug, ThisError)]
	pub enum Error {
		#[error("Serde json error")]
		SerdeJson(#[from] SerdeJsonError),
		#[error("Isahc error")]
		Isahc(#[from] IsahcError),
	}

	#[derive(Debug, ThisError)]
	pub enum SerdeJsonError {
		#[error("Raw serde json error")]
		Raw(#[from] RawSerdeJsonError),
	}

	pub async fn send_rpc(uri: impl AsRef<str>, body: Value) -> SubrpcerResult<IsahcResponse> {
		let mut request_builder = RequestBuilder::new()
			.method(HttpMethod::POST)
			.uri(uri.as_ref());

		request_builder.headers_mut().unwrap().append(
			CONTENT_TYPE,
			"application/json;charset=utf-8".parse().unwrap(),
		);

		let request = request_builder
			.body(serde_json::to_vec(&body).map_err(SerdeJsonError::from)?)
			.unwrap();
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

const DEFAULT_ID: u8 = 1;

pub fn rpc(method: impl Serialize, params: impl Serialize, id: impl Serialize) -> Value {
	json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}
