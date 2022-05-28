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

/// Spawn a Substrate standard node.
pub fn spawn(executable: &str, rpc_port: u16, chain: &str) -> Result<Child> {
	let mut node = Command::new(executable)
		.stdout(Stdio::null())
		.stderr(Stdio::piped())
		.args(&[&format!("--rpc-port={rpc_port}"), "--chain", &format!("{}-dev", chain), "--tmp"])
		.spawn()
		.map_err(|e| error::Node::StartFailed(e))?;
	let output =
		BufReader::new(node.stderr.take().ok_or_else(|| {
			error::Generic::AlmostImpossibleError("[core::node `stderr` is empty]")
		})?);

	// Ensure the node is fully startup.
	for line in output.lines().filter_map(::std::io::Result::ok) {
		if line.contains("Idle") {
			break;
		}
	}

	Ok(node)
}

/// Fetch the runtime version from a node.
pub async fn runtime_version(uri: &str) -> Result<RuntimeVersion> {
	let result = http::send_rpc(uri, &state::get_runtime_version())
		.await?
		.json::<Value>()
		.await
		.map_err(error::Generic::from)?
		.get_mut("result")
		.ok_or(error::Node::GetRpcResultFailed)?
		.take();
	let runtime_version = serde_json::from_value(result).map_err(error::Generic::from)?;

	Ok(runtime_version)
}

/// Fetch the runtime metadata from a node.
pub async fn runtime_metadata(uri: &str) -> Result<LatestRuntimeMetadata> {
	let result = http::send_rpc(uri, &state::get_metadata())
		.await?
		.json::<Value>()
		.await
		.map_err(error::Generic::from)?
		.get_mut("result")
		.ok_or(error::Node::GetRpcResultFailed)?
		.take();
	let hex_codec_metadata = result.as_str().ok_or(error::Generic::AlmostImpossibleError(
		"[core::node] `hex_codec_metadata` is non-str",
	))?;
	let codec_metadata = array_bytes::hex2bytes(hex_codec_metadata).map_err(|_| {
		error::Generic::AlmostImpossibleError("[core::node] `codec_metadata` is non-hex")
	})?;
	let metadata_prefixed =
		RuntimeMetadataPrefixed::decode(&mut &*codec_metadata).map_err(error::Generic::from)?;
	let metadata = submetadatan::metadata(metadata_prefixed)
		.map_err(|e| error::Node::ParseMetadataFailed(e))?;

	Ok(metadata)
}
