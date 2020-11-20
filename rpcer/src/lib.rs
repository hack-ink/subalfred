pub async fn send_rpc(
	address: impl AsRef<str>,
	method: impl AsRef<str>,
	params: Value,
) -> Result<IsahcResponse> {
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
	let result = isahc::send_async(request).await?;

	trace!("{:#?}", result);

	Ok(result)
}
