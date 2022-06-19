//! The core library about how Subalfred interacts with Substrate-based node.

/// TODO
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
const E_INVALID_PROGRESS_BAR_TEMPLATE: &str = "[core::node] invalid progress bar template]";

/// Spawn a Substrate standard node.
pub fn spawn(executable: &str, rpc_port: u16, chain: &str) -> Result<Child> {
	let mut node = Command::new(executable)
		.stdout(Stdio::null())
		.stderr(Stdio::piped())
		.args(&[&format!("--rpc-port={rpc_port}"), "--chain", &format!("{}-dev", chain), "--tmp"])
		.spawn()
		.map_err(error::Node::StartNodeFailed)?;
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

/// Get the runtime version from a node.
pub async fn runtime_version(uri: &str) -> Result<RuntimeVersion> {
	Ok(http::send::<_, RuntimeVersion>(uri, &state::get_runtime_version_once()).await?.result)
}

/// Fetch the runtime metadata from a node.
pub async fn runtime_metadata(uri: &str) -> Result<LatestRuntimeMetadata> {
	let response = http::send::<_, String>(uri, &state::get_metadata_once()).await?;
	let codec_metadata = array_bytes::hex2bytes(&response.result)
		.map_err(|_| error::Generic::AlmostImpossible(E_CODEC_METADATA_IS_NON_HEX))?;
	let metadata_prefixed =
		RuntimeMetadataPrefixed::decode(&mut &*codec_metadata).map_err(error::Generic::Codec)?;
	let metadata =
		submetadatan::metadata(metadata_prefixed).map_err(error::Node::ParseMetadataFailed)?;

	Ok(metadata)
}
