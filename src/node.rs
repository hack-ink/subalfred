// --- crates.io ---
use isahc::{
	http::{header::CONTENT_TYPE, request::Builder as RequestBuilder, Method as HttpMethod},
	ResponseExt,
};
use serde::Deserialize;
use serde_json::{json, Value};
// --- subalfred ---
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct RpcResult {
	id: u32,
	jsonrpc: String,
	result: Value,
}

pub async fn send_rpc(
	address: impl AsRef<str>,
	method: impl AsRef<str>,
	params: Value,
) -> Result<RpcResult> {
	let mut request_builder = RequestBuilder::new()
		.method(HttpMethod::POST)
		.uri(address.as_ref());

	request_builder.headers_mut().unwrap().append(
		CONTENT_TYPE,
		"application/json;charset=utf-8".parse().unwrap(),
	);

	let body = json!({
		"jsonrpc": "2.0",
		"id": 1,
		"method": method.as_ref(),
		"params": params
	});
	let request = request_builder.body(body.to_string()).unwrap();
	let result: RpcResult = isahc::send_async(request).await?.json()?;

	#[cfg(feature = "dbg")]
	dbg!(&result);

	Ok(result)
}
