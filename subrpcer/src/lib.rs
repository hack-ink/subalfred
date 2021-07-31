pub mod author;
pub mod chain;
pub mod grandpa;
pub mod state;
pub mod system;

#[cfg(feature = "client")]
pub mod client {
	pub use isahc;

	// --- crates.io ---
	use isahc::{
		http::{
			header::CONTENT_TYPE, request::Builder as RequestBuilder, Method as HttpMethod,
			Response,
		},
		AsyncBody as IsahcAsyncBody, Body as IsahcBody, Error as IsahcError,
	};
	use serde_json::{Error as RawSerdeJsonError, Value};
	use thiserror::Error as ThisError;
	use tracing::trace;

	pub type SubrpcerResult<T> = Result<T, Error>;
	pub type IsahcResponse = Response<IsahcBody>;
	pub type IsahcAsyncResponse = Response<IsahcAsyncBody>;

	#[derive(Debug, ThisError)]
	#[error("{0}")]
	pub enum Error {
		SerdeJson(#[from] SerdeJsonError),
		Isahc(#[from] IsahcError),
	}

	#[derive(Debug, ThisError)]
	#[error("{0}")]
	pub enum SerdeJsonError {
		Raw(#[from] RawSerdeJsonError),
	}

	pub fn send_rpc(uri: impl AsRef<str>, body: Value) -> SubrpcerResult<IsahcResponse> {
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
		let result = isahc::send(request)?;

		trace!("{:#?}", result);

		Ok(result)
	}

	pub async fn send_rpc_async(
		uri: impl AsRef<str>,
		body: Value,
	) -> SubrpcerResult<IsahcAsyncResponse> {
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
#[cfg(feature = "client")]
pub use client::*;

// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};

const DEFAULT_ID: u8 = 1;

pub fn rpc(id: impl Serialize, method: impl Serialize, params: impl Serialize) -> Value {
	json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}

#[cfg(feature = "tracing")]
pub fn debug_rpc(rpc: Value) -> Value {
	tracing::debug!("{}", rpc);

	rpc
}
