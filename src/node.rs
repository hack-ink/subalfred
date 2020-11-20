// --- std ---
use std::{fmt::Debug, fs::File, io::Read};
// --- crates.io ---
use isahc::{
	http::{
		header::CONTENT_TYPE, request::Builder as RequestBuilder, Method as HttpMethod, Response,
	},
	Body as IsahcBody, ResponseExt,
};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Value};
use tracing::trace;
// --- subalfred ---
use crate::{config::Runtime, Result, Subalfred};

type IsahcResponse = Response<IsahcBody>;

#[derive(Debug, Deserialize)]
pub struct RpcResult {
	pub result: Value,
}
impl RpcResult {
	pub fn into_inner<T: DeserializeOwned>(self) -> T {
		serde_json::from_value(self.result).unwrap()
	}
}

#[derive(Debug)]
pub struct RuntimeVersions {
	chain_runtime_version: RuntimeVersion,
	github_runtime_version: RuntimeVersion,
	local_runtime_version: RuntimeVersion,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
	pub spec_name: String,
	pub impl_name: String,
	pub authoring_version: u32,
	pub spec_version: u32,
	pub impl_version: u32,
	pub transaction_version: u32,
}

impl Subalfred {
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

	pub async fn check_runtime_version(&self) -> Result<Vec<Vec<RuntimeVersions>>> {
		let mut runtimes = <Vec<Vec<RuntimeVersions>>>::new();

		for Runtime {
			runtime_relative_path,
			node_rpc_address,
		} in &self.project.runtimes
		{
			let chain_runtime_version = serde_json::from_value::<RuntimeVersion>(
				Self::send_rpc(
					node_rpc_address,
					"state_getRuntimeVersion",
					serde_json::from_str("[]").unwrap(),
				)
				.await?
				.json::<RpcResult>()?
				.result,
			)
			.unwrap();
			let github_runtime_version = {
				let download_url = self
					.get_repository_content(
						&self.project.owner,
						&self.project.repo,
						runtime_relative_path,
						None,
					)
					.await?
					.download_url;

				extract_runtime_version(&self.githuber.download(download_url).await?.text()?)
			};
			let local_runtime_version = {
				let mut f = File::open(&format!(
					"{}/{}",
					&self.project.local_full_path, runtime_relative_path
				))
				.unwrap();
				let mut s = String::new();

				f.read_to_string(&mut s).unwrap();

				extract_runtime_version(&s)
			};
			let runtime_versions = RuntimeVersions {
				chain_runtime_version,
				github_runtime_version,
				local_runtime_version,
			};

			if let Some(i) = runtimes.iter().position(|runtimes| {
				&runtimes[0].chain_runtime_version.spec_name
					== &runtime_versions.chain_runtime_version.spec_name
			}) {
				runtimes[i].push(runtime_versions);
			} else {
				runtimes.push(vec![runtime_versions]);
			}
		}

		trace!("{:#?}", runtimes);

		Ok(runtimes)
	}
}

fn extract_runtime_version(s: &str) -> RuntimeVersion {
	let extract_name = |s| format!(r#"{} *?: *?create_runtime_str! *?\("(.+?)"\)"#, s);
	let extract_version = |s| format!(r#"{} *?: *?(\d+)"#, s);

	let runtime_version_extractor = regex::Regex::new(
		r#"pub +?const +?VERSION *?: +?RuntimeVersion +?= +?RuntimeVersion +?\{(?s)(.+?)\}"#,
	)
	.unwrap();
	let spec_name_extractor = regex::Regex::new(&extract_name("spec_name")).unwrap();
	let impl_name_extractor = regex::Regex::new(&extract_name("impl_name")).unwrap();
	let authoring_version_extractor =
		regex::Regex::new(&extract_version("authoring_version")).unwrap();
	let spec_version_extractor = regex::Regex::new(&extract_version("spec_version")).unwrap();
	let impl_version_extractor = regex::Regex::new(&extract_version("impl_version")).unwrap();
	let transaction_version_extractor =
		regex::Regex::new(&extract_version("transaction_version")).unwrap();

	let runtime_version = &runtime_version_extractor.captures(&s).unwrap()[0];
	let spec_name = spec_name_extractor.captures(&runtime_version).unwrap()[1].to_string();
	let impl_name = impl_name_extractor.captures(&runtime_version).unwrap()[1].to_string();
	let authoring_version = authoring_version_extractor
		.captures(&runtime_version)
		.unwrap()[1]
		.parse()
		.unwrap();
	let spec_version = spec_version_extractor.captures(&runtime_version).unwrap()[1]
		.parse()
		.unwrap();
	let impl_version = impl_version_extractor.captures(&runtime_version).unwrap()[1]
		.parse()
		.unwrap();
	let transaction_version = transaction_version_extractor
		.captures(&runtime_version)
		.unwrap()[1]
		.parse()
		.unwrap();

	let runtime_version = RuntimeVersion {
		spec_name,
		impl_name,
		authoring_version,
		spec_version,
		impl_version,
		transaction_version,
	};

	trace!("{:#?}", runtime_version);

	runtime_version
}
