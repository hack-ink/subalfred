//! Subalfred core node library.

// std
use std::{
	io::{BufRead, BufReader},
	process::{Child, Command, Stdio},
};
// crates.io
use array_bytes::TryFromHex;
use parity_scale_codec::Decode;
// hack-ink
use crate::{jsonrpc::http, prelude::*};
use submetadatan::{LatestRuntimeMetadata, RuntimeMetadataPrefixed};
use subrpcer::state;
use subversioner::RuntimeVersion;

const E_BLOCK_NUMBER_IS_NON_HEX: &str =
	"[core::node] block number is non-hex, maybe the Substrate RPC SPEC changed";
const E_CODEC_METADATA_IS_NON_HEX: &str =
	"[core::node] `codec_metadata` is non-hex, maybe the Substrate RPC SPEC changed";
const E_STDERR_IS_EMPTY: &str =
	"[core::node] `stderr` is empty, , maybe the substrate node behavior changed";

/// Spawn a Substrate-Base standard node.
pub fn spawn(executable: &str, rpc_port: u16, chain: &str) -> Result<Child> {
	let mut node = Command::new(executable)
		.stdout(Stdio::null())
		.stderr(Stdio::piped())
		.args([&format!("--rpc-port={rpc_port}"), "--chain", chain, "--tmp"])
		.spawn()
		.map_err(error::Node::StartNodeFailed)?;
	let output = BufReader::new(
		node.stderr.take().ok_or_else(|| error::almost_impossible(E_STDERR_IS_EMPTY))?,
	);

	// Ensure the node is fully startup.
	for line in output.lines() {
		let line = line.map_err(error::Generic::Io)?;

		tracing::trace!("node({rpc_port}) {line}");

		if ["Idle", "Imported", "Syncing"].iter().any(|s| line.contains(s)) {
			break;
		}
	}

	Ok(node)
}

/// Get runtime version from node.
pub async fn runtime_version(uri: &str) -> Result<RuntimeVersion> {
	Ok(http::send::<_, RuntimeVersion>(uri, &state::get_runtime_version(0, None::<()>))
		.await?
		.result)
}

/// Fetch runtime metadata from node.
pub async fn runtime_metadata(uri: &str) -> Result<LatestRuntimeMetadata> {
	let response = http::send::<_, String>(uri, &state::get_metadata(0, None::<()>)).await?;

	parse_raw_runtime_metadata(&response.result)
}
/// Parse the raw metadata.
pub fn parse_raw_runtime_metadata(raw_runtime_metadata: &str) -> Result<LatestRuntimeMetadata> {
	let codec_metadata = array_bytes::hex2bytes(raw_runtime_metadata)
		.map_err(|_| error::almost_impossible(E_CODEC_METADATA_IS_NON_HEX))?;
	let metadata_prefixed =
		RuntimeMetadataPrefixed::decode(&mut &*codec_metadata).map_err(error::Generic::Codec)?;
	let metadata =
		submetadatan::metadata(metadata_prefixed).map_err(error::Node::ParseMetadataFailed)?;

	Ok(metadata)
}

// TODO: move to somewhere
/// Find the runtime upgrade that happened at which block with the dichotomy algorithm.
pub async fn find_runtime_upgrade_block(
	runtime_version: u32,
	uri: &str,
) -> Result<Option<(u32, String)>> {
	// subalfred
	use crate::{
		jsonrpc::ws::Initializer,
		substrate_client::{BasicApi, Client},
	};

	let client = Client::initialize(Initializer::new(), uri).await?;
	let best_finalized_hash = client.get_finalized_head().await?;
	let mut left = 0;
	let mut right =
		u32::try_from_hex(&client.get_header::<String, _>(Some(best_finalized_hash)).await?.number)
			.map_err(|_| error::almost_impossible(E_BLOCK_NUMBER_IS_NON_HEX))?;
	let mut mid = right / 2;

	loop {
		let block_hash = client.get_block_hash(Some(mid)).await?;
		let fetched_runtime_version =
			client.get_runtime_version(Some(&block_hash)).await?.spec_version;

		tracing::trace!("({left}, {right}) -> {fetched_runtime_version}");

		if left == mid || right == mid {
			let block_number = mid + 1;
			let block_hash = client.get_block_hash(Some(block_number)).await?;
			let fetched_runtime_version =
				client.get_runtime_version(Some(&block_hash)).await?.spec_version;

			if fetched_runtime_version == runtime_version {
				return Ok(Some((block_number, block_hash)));
			} else {
				return Ok(None);
			}
		}

		if fetched_runtime_version >= runtime_version {
			right = mid;
			mid -= (mid - left) / 2;
		} else {
			left = mid;
			mid += (right - mid) / 2;
		}
	}
}
