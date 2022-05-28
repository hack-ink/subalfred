use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error(transparent)]
	Cargo(#[from] Cargo),
	#[error(transparent)]
	Generic(#[from] Generic),
	#[error(transparent)]
	Node(#[from] Node),
	#[error(transparent)]
	Ss58(#[from] Ss58),
}

#[derive(Debug, ThisError)]
pub enum Cargo {
	#[error("[core::cargo] failed to exec `cargo metadata`, {0:?}")]
	ExecMetadataFailed(#[source] cargo_metadata::Error),
	#[error("[core::cargo] failed to open the manifest file, {0:?}")]
	OpenManifestFailed(#[source] cargo_toml::Error),
}

#[derive(Debug, ThisError)]
pub enum Generic {
	#[error("{0:?}")]
	AlmostImpossibleError(&'static str),
	#[error(transparent)]
	CodecError(#[from] parity_scale_codec::Error),
	#[error(transparent)]
	ReqwestError(#[from] reqwest::Error),
	#[error(transparent)]
	SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, ThisError)]
pub enum Node {
	#[error("[core::node] failed to get the RPC result")]
	GetRpcResultFailed,
	#[error("[core::node] failed to parse metadata, {0:?}")]
	ParseMetadataFailed(#[source] submetadatan::Error),
	#[error("[core::node] failed to start the node, {0:?}")]
	StartFailed(#[source] std::io::Error),
}

#[derive(Debug, ThisError)]
pub enum Ss58 {
	#[error("[core::ss58] invalid address, {address:?}")]
	InvalidAddress { address: String, source: Option<Ss58InvalidAddressSource> },
	#[error("[core::ss58] failed to calculate SS58 address, {0:?}")]
	CalculateSs58AddressFailed(#[source] subcryptor::Error),
}
#[derive(Debug, ThisError)]
pub enum Ss58InvalidAddressSource {
	#[error("{0:?}")]
	ArrayBytes(array_bytes::Error),
	#[error(transparent)]
	Subcryptor(subcryptor::Error),
}
