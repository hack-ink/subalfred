// --- std ---
use std::{fs::File, io::Read};
// --- crates.io ---
use async_std::sync::Arc;
use githubman::Githubman;
use isahc::{
	http::{header::CONTENT_TYPE, request::Builder as RequestBuilder, Method as HttpMethod},
	ResponseExt,
};
use serde::Deserialize;
use serde_json::{json, Value};
// --- subalfred ---
use crate::{
	config::{Runtime, CONFIG},
	Result,
};

#[derive(Debug, Deserialize)]
pub struct RpcResult {
	pub id: u32,
	pub jsonrpc: String,
	pub result: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
	pub authoring_version: u32,
	pub impl_name: String,
	pub impl_version: u32,
	pub spec_name: String,
	pub spec_version: u32,
	pub transaction_version: u32,
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

pub async fn check_runtime_version(githubman: &Arc<Githubman>) -> Result<()> {
	for Runtime {
		runtime_relative_path,
		node_rpc_address,
	} in &CONFIG.substrate_project.runtimes
	{
		let runtime_version = serde_json::to_string_pretty(
			&send_rpc(
				node_rpc_address,
				"state_getRuntimeVersion",
				serde_json::from_str("[]").unwrap(),
			)
			.await?
			.result,
		)
		.unwrap();

		#[cfg(feature = "dbg")]
		dbg!(runtime_version);

		// let s = githubman.send(request).await?.text().unwrap();

		let mut f = File::open(&format!(
			"{}/{}",
			&CONFIG.substrate_project.local_full_path, runtime_relative_path
		))
		.unwrap();
		let mut s = String::new();

		f.read_to_string(&mut s).unwrap();
		extract_runtime_version(&s);
	}

	Ok(())
}

fn extract_runtime_version(s: &str) {
	let extract_name = |s| format!(r#"{} *?: *?create_runtime_str! *?\("(.+?)"\)"#, s);
	let extract_version = |s| format!(r#"{} *?: *?(\d+)"#, s);

	let re = regex::Regex::new(
		r#"pub +?const +?VERSION *?: +?RuntimeVersion +?= +?RuntimeVersion +?\{(?s)(.+?)\}"#,
	)
	.unwrap();
	let s = &re.captures(&s).unwrap()[0];

	#[cfg(feature = "dbg")]
	dbg!(s);

	let re = regex::Regex::new(&extract_name("spec_name")).unwrap();
	let spec_name = &re.captures(&s).unwrap()[1];

	#[cfg(feature = "dbg")]
	dbg!(spec_name);

	let re = regex::Regex::new(&extract_name("impl_name")).unwrap();
	let impl_name = &re.captures(&s).unwrap()[1];

	#[cfg(feature = "dbg")]
	dbg!(impl_name);

	let re = regex::Regex::new(&extract_version("authoring_version")).unwrap();
	let authoring_version = &re.captures(&s).unwrap()[1];

	#[cfg(feature = "dbg")]
	dbg!(authoring_version);

	let re = regex::Regex::new(&extract_version("spec_version")).unwrap();
	let spec_version = &re.captures(&s).unwrap()[1];

	#[cfg(feature = "dbg")]
	dbg!(spec_version);

	let re = regex::Regex::new(&extract_version("impl_version")).unwrap();
	let impl_version = &re.captures(&s).unwrap()[1];

	#[cfg(feature = "dbg")]
	dbg!(impl_version);

	let re = regex::Regex::new(&extract_version("transaction_version")).unwrap();
	let transaction_version = &re.captures(&s).unwrap()[1];

	#[cfg(feature = "dbg")]
	dbg!(transaction_version);
}
