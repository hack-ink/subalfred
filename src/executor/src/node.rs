// std
use std::{
	fmt::Debug,
	io::{BufRead, BufReader},
	process::{Child, Command, Stdio},
};
// crates.io
use parity_scale_codec::Decode;
use serde::Deserialize;
use serde_json::Value;
// hack-ink
use crate::*;
use submetadatan::{LatestRuntimeMetadata, RuntimeMetadataPrefixed};
use subrpcer::client::u;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
	pub spec_name: String,
	pub impl_name: String,
	pub authoring_version: u32,
	pub spec_version: u32,
	pub impl_version: u32,
	pub transaction_version: u32,
}

#[derive(Clone, Debug)]
pub enum ChainType<T> {
	Local(T),
	Live(T),
}
impl<T> ChainType<T> {
	pub fn local(t: T) -> Self {
		Self::Local(t)
	}

	pub fn live(t: T) -> Self {
		Self::Live(t)
	}

	pub fn wrap<'a, F>(values: &'a [T], f: F) -> Vec<ChainType<&'a T>>
	where
		F: Fn(&'a T) -> ChainType<&'a T>,
	{
		values.iter().map(f).collect()
	}

	pub fn inner(&self) -> &T {
		match self {
			ChainType::Local(t) => t,
			ChainType::Live(t) => t,
		}
	}

	pub fn is_live(&self) -> bool {
		match self {
			ChainType::Live(_) => true,
			_ => false,
		}
	}

	pub fn opposite_to(&self, other: &Self) -> bool {
		if self.is_live() {
			!other.is_live()
		} else {
			other.is_live()
		}
	}

	pub fn output<'a, 'b, F, D>(&'a self, prefix: &str, f: F) -> String
	where
		'a: 'b,
		F: FnOnce(&'b T) -> D,
		D: Debug,
	{
		if self.is_live() {
			format!("{}", format!("- {}: {:?}", prefix, f(self.inner())))
		} else {
			format!("{}", format!("+ {}: {:?}", prefix, f(self.inner())))
		}
	}
}

impl Executor {
	pub fn spawn_local_node(executable: &str, chain: &str) -> AnyResult<Child> {
		let mut local_node = Command::new(executable)
			.stderr(Stdio::piped())
			.args(&["--chain", &format!("{}-dev", chain), "--tmp"])
			.spawn()?;
		let output = BufReader::new(local_node.stderr.take().unwrap());

		for line in output.lines().filter_map(Result::ok) {
			if line.contains("Idle") {
				break;
			}
		}

		Ok(local_node)
	}

	pub fn fetch_runtime_version(uri: impl AsRef<str>) -> AnyResult<RuntimeVersion> {
		let result = u::send_rpc(uri, &subrpcer::state::get_runtime_version())?
			.into_json::<Value>()?
			.get_mut("result")
			.ok_or(anyhow::anyhow!("Can NOT find field `result` in JSON"))?
			.take();
		let runtime_version = serde_json::from_value(result)?;

		Ok(runtime_version)
	}

	pub fn fetch_metadata(uri: impl AsRef<str>) -> AnyResult<LatestRuntimeMetadata> {
		let mut response =
			u::send_rpc(uri, &subrpcer::state::get_metadata())?.into_json::<Value>()?;
		let hex_codec_metadata = response
			.get_mut("result")
			.map(|v| v.take())
			.ok_or(anyhow::anyhow!("Can NOT find field `result` in JSON"))?
			.as_str()
			.map(ToOwned::to_owned)
			.ok_or(anyhow::anyhow!("Can NOT convert `result` to `str`"))?;
		let codec_metadata =
			array_bytes::hex2bytes(hex_codec_metadata).map_err(|e| anyhow::anyhow!("{:?}", e))?;
		let metadata_prefixed = RuntimeMetadataPrefixed::decode(&mut &*codec_metadata)?;
		let metadata = submetadatan::metadata(metadata_prefixed)?;

		Ok(metadata)
	}
}
