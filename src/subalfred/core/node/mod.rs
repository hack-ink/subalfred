// std
use std::{
	io::{BufRead, BufReader},
	process::{Child, Command, Stdio},
};
// crates.io
use parity_scale_codec::Decode;
use serde_json::Value;
// hack-ink
use crate::core::{error, http, Result};
use submetadatan::{LatestRuntimeMetadata, RuntimeMetadataPrefixed};
use subrpcer::state;
use subversion::RuntimeVersion;

const E_CODEC_METADATA_IS_NON_HEX: &str = "[core::node] `codec_metadata` is non-hex";
const E_HEX_METADATA_IS_NON_STR: &str = "[core::node] `hex_codec_metadata` is non-str";
const E_STDERR_IS_EMPTY: &str = "[core::node `stderr` is empty]";

/// Spawn a Substrate standard node.
pub fn spawn(executable: &str, rpc_port: u16, chain: &str) -> Result<Child> {
	let mut node = Command::new(executable)
		.stdout(Stdio::null())
		.stderr(Stdio::piped())
		.args(&[&format!("--rpc-port={rpc_port}"), "--chain", &format!("{}-dev", chain), "--tmp"])
		.spawn()
		.map_err(error::Node::StartFailed)?;
	let output = BufReader::new(
		node.stderr.take().ok_or(error::Generic::AlmostImpossible(E_STDERR_IS_EMPTY))?,
	);

	// Ensure the node is fully startup.
	// TODO: emit the error or not
	for line in output.lines().filter_map(::std::io::Result::ok) {
		if line.contains("Idle") {
			break;
		}
	}

	Ok(node)
}

/// Fetch the runtime version from a node.
pub async fn runtime_version(uri: &str) -> Result<RuntimeVersion> {
	let result = http::send_jsonrpc(uri, &state::get_runtime_version_once())
		.await?
		.json::<Value>()
		.await
		.map_err(error::Generic::Reqwest)?
		.get_mut("result")
		.ok_or(error::Node::GetRpcResultFailed)?
		.take();
	let runtime_version = serde_json::from_value(result).map_err(error::Generic::Serde)?;

	Ok(runtime_version)
}

/// Fetch the runtime metadata from a node.
pub async fn runtime_metadata(uri: &str) -> Result<LatestRuntimeMetadata> {
	let result = http::send_jsonrpc(uri, &state::get_metadata_once())
		.await?
		.json::<Value>()
		.await
		.map_err(error::Generic::Reqwest)?
		.get_mut("result")
		.ok_or(error::Node::GetRpcResultFailed)?
		.take();
	let hex_codec_metadata =
		result.as_str().ok_or(error::Generic::AlmostImpossible(E_HEX_METADATA_IS_NON_STR))?;
	let codec_metadata = array_bytes::hex2bytes(hex_codec_metadata)
		.map_err(|_| error::Generic::AlmostImpossible(E_CODEC_METADATA_IS_NON_HEX))?;
	let metadata_prefixed =
		RuntimeMetadataPrefixed::decode(&mut &*codec_metadata).map_err(error::Generic::Codec)?;
	let metadata =
		submetadatan::metadata(metadata_prefixed).map_err(error::Node::ParseMetadataFailed)?;

	Ok(metadata)
}
