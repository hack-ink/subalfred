// --- std ---
use std::{fmt::Debug, fs::File, io::Read};
// --- crates.io ---
use isahc::AsyncReadResponseExt;
use regex::Regex;
use serde::Deserialize;
use subrpcer::client::i;
// --- subalfred ---
use crate::{config::Runtime, substrate::node::RpcResult, AnyResult, Subalfred};

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
	pub async fn check_runtime_version(&self) -> AnyResult<Vec<RuntimeVersions>> {
		let mut runtimes = vec![];

		for Runtime {
			runtime_relative_path,
			node_rpc_uri,
		} in &self.project.runtimes
		{
			let result = {
				let mut v = vec![];
				i::send_rpc_async(node_rpc_uri, subrpcer::state::get_runtime_version())
					.await?
					.copy_to(&mut v)
					.await?;

				serde_json::from_slice::<RpcResult>(&v)?.result
			};
			let chain_runtime_version = serde_json::from_value::<RuntimeVersion>(result).unwrap();
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

				extract_runtime_version(&self.githuber.download(download_url).await?.text().await?)
			};
			let local_runtime_version = {
				let path = format!(
					"{}/{}",
					&self.project.local_full_path, runtime_relative_path
				);

				tracing::trace!("{}", path);

				let mut f = File::open(&path).unwrap();
				let mut s = String::new();

				f.read_to_string(&mut s).unwrap();

				extract_runtime_version(&s)
			};

			runtimes.push(RuntimeVersions {
				chain_runtime_version,
				github_runtime_version,
				local_runtime_version,
			});
		}

		tracing::trace!("{:#?}", runtimes);

		Ok(runtimes)
	}
}

fn extract_runtime_version(s: &str) -> RuntimeVersion {
	let extract_name = |s| format!(r#".*?{} *?:.*?"(.+?)""#, s);
	let extract_version = |s| format!(r#".*?{} *?: *?(\d+)"#, s);

	let runtime_version_extractor = Regex::new(
		r#"pub +?const +?VERSION *?: +?RuntimeVersion +?= +?RuntimeVersion +?\{(?s)(.+?)\}"#,
	)
	.unwrap();
	let spec_name_extractor = Regex::new(&extract_name("spec_name")).unwrap();
	let impl_name_extractor = Regex::new(&extract_name("impl_name")).unwrap();
	let authoring_version_extractor = Regex::new(&extract_version("authoring_version")).unwrap();
	let spec_version_extractor = Regex::new(&extract_version("spec_version")).unwrap();
	let impl_version_extractor = Regex::new(&extract_version("impl_version")).unwrap();
	let transaction_version_extractor =
		Regex::new(&extract_version("transaction_version")).unwrap();

	let runtime_version = &runtime_version_extractor.captures(&s).unwrap()[0];

	tracing::trace!("{}", runtime_version);

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

	tracing::trace!("{:#?}", runtime_version);

	runtime_version
}
