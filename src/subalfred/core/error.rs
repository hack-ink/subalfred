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
	IoError(#[from] std::io::Error),
	#[error(transparent)]
	ReqwestError(#[from] reqwest::Error),
	#[error(transparent)]
	SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, ThisError)]
pub enum Node {
	#[error("[core::node] failed to start the node, {0:?}")]
	StartFailed(#[source] std::io::Error),
	#[error("[core::node] failed to get the RPC result")]
	GetRpcResultFailed,
}

#[derive(Debug, ThisError)]
pub enum Ss58 {
	#[error("[core::ss58] invalid address, {0:?}")]
	InvalidAddress(String),
	#[error("[core::ss58] unsupported network, {0:?}")]
	UnsupportedNetwork(String),
}
