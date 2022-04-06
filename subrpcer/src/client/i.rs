pub use isahc::{self, Error as IsahcError};

// crates.io
use isahc::{
	http::{
		header::CONTENT_TYPE, request::Builder as RequestBuilder, Method as HttpMethod, Response,
	},
	AsyncBody as IsahcAsyncBody, Body as IsahcBody,
};
use serde_json::Value;
// hack-ink
use crate::client::Error;

pub type SubrpcerResult<T> = Result<T, Error>;
pub type IsahcResponse = Response<IsahcBody>;
pub type IsahcAsyncResponse = Response<IsahcAsyncBody>;

pub fn send_rpc(uri: impl AsRef<str>, body: &Value) -> SubrpcerResult<IsahcResponse> {
	let mut request_builder = RequestBuilder::new()
		.method(HttpMethod::POST)
		.uri(uri.as_ref());

	request_builder.headers_mut().unwrap().append(
		CONTENT_TYPE,
		"application/json;charset=utf-8".parse().unwrap(),
	);

	let request = request_builder.body(serde_json::to_vec(body)?).unwrap();
	let result = isahc::send(request)?;

	tracing::trace!("{:#?}", result);

	Ok(result)
}

pub async fn send_rpc_async(
	uri: impl AsRef<str>,
	body: &Value,
) -> SubrpcerResult<IsahcAsyncResponse> {
	let mut request_builder = RequestBuilder::new()
		.method(HttpMethod::POST)
		.uri(uri.as_ref());

	request_builder.headers_mut().unwrap().append(
		CONTENT_TYPE,
		"application/json;charset=utf-8".parse().unwrap(),
	);

	let request = request_builder.body(serde_json::to_vec(body)?).unwrap();
	let result = isahc::send_async(request).await?;

	tracing::trace!("{:#?}", result);

	Ok(result)
}
