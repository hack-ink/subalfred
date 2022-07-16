//! The core library about how Subalfred interacts with Substrate-based node.

pub mod export_state;

// std
use std::{
	io::{BufRead, BufReader},
	process::{Child, Command, Stdio},
};
// crates.io
use parity_scale_codec::Decode;
// hack-ink
use crate::core::{error, jsonrpc::http, Result};
use submetadatan::{LatestRuntimeMetadata, RuntimeMetadataPrefixed};
use subrpcer::state;
use subversion::RuntimeVersion;

const E_CODEC_METADATA_IS_NON_HEX: &str = "[core::node] `codec_metadata` is non-hex";
const E_STDERR_IS_EMPTY: &str = "[core::node] `stderr` is empty";

/// Spawn a Substrate-Base standard node.
pub fn spawn(executable: &str, rpc_port: u16, chain: &str) -> Result<Child> {
	let mut node = Command::new(executable)
		.stdout(Stdio::null())
		.stderr(Stdio::piped())
		.args(&[&format!("--rpc-port={rpc_port}"), "--chain", &format!("{}-dev", chain), "--tmp"])
		.spawn()
		.map_err(error::Node::StartNodeFailed)?;
	let output = BufReader::new(
		node.stderr.take().ok_or_else(|| error::almost_impossible(E_STDERR_IS_EMPTY))?,
	);

	// Ensure the node is fully startup.
	for line in output.lines() {
		if line.map_err(error::Generic::Io)?.contains("Idle") {
			break;
		}
	}

	Ok(node)
}

/// Get runtime version from node.
pub async fn runtime_version(uri: &str) -> Result<RuntimeVersion> {
	Ok(http::send::<_, RuntimeVersion>(uri, &state::get_runtime_version_once()).await?.result)
}

/// Fetch runtime metadata from node.
pub async fn runtime_metadata(uri: &str) -> Result<LatestRuntimeMetadata> {
	let response = http::send::<_, String>(uri, &state::get_metadata_once()).await?;

	parse_raw_runtime_metadata(&response.result)
}
fn parse_raw_runtime_metadata(raw_runtime_metadata: &str) -> Result<LatestRuntimeMetadata> {
	let codec_metadata = array_bytes::hex2bytes(raw_runtime_metadata)
		.map_err(|_| error::almost_impossible(E_CODEC_METADATA_IS_NON_HEX))?;
	let metadata_prefixed =
		RuntimeMetadataPrefixed::decode(&mut &*codec_metadata).map_err(error::Generic::Codec)?;
	let metadata =
		submetadatan::metadata(metadata_prefixed).map_err(error::Node::ParseMetadataFailed)?;

	Ok(metadata)
}
