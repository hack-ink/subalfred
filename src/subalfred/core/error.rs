use thiserror::Error as ThisError;

/// The core libraries' main `Error` type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Error {
	#[error(transparent)]
	Debug(#[from] Debug),

	#[error(transparent)]
	Cargo(#[from] Cargo),
	#[error(transparent)]
	Generic(#[from] Generic),
	#[error(transparent)]
	Jsonrpc(#[from] Jsonrpc),
	#[error(transparent)]
	Key(#[from] Key),
	#[error(transparent)]
	Node(#[from] Node),
	#[error(transparent)]
	Ss58(#[from] Ss58),
	#[error(transparent)]
	Tokio(#[from] Tokio),
}

/// Print the error directly.
#[derive(Debug)]
pub struct Debug(String);
impl std::fmt::Display for Debug {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}
impl std::error::Error for Debug {}
/// Convert the error to [`struct@Debug`].
pub fn quick_debug<E>(e: E) -> Debug
where
	E: std::fmt::Debug,
{
	Debug(format!("{e:?}"))
}

/// The core Cargo error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Cargo {
	#[error("[core::cargo] failed to exec `cargo metadata`")]
	ExecMetadataFailed(#[source] cargo_metadata::Error),
	#[error("[core::cargo] failed to open the manifest file")]
	OpenManifestFailed(#[source] cargo_toml::Error),
}

/// The generic error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Generic {
	#[error("{0:?}")]
	AlmostImpossible(&'static str),
	#[error(transparent)]
	Codec(#[from] parity_scale_codec::Error),
	#[error(transparent)]
	Fmt(#[from] std::fmt::Error),
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),
	#[error(transparent)]
	Serde(#[from] serde_json::Error),
	#[error(transparent)]
	Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
}

/// The core JSONRPC error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Jsonrpc {
	#[error("[core::jsonrpc] empty batch")]
	EmptyBatch,
	#[error("[core::jsonrpc] exceeded the maximum number of request queue size, {0}")]
	ExceededRequestQueueMaxSize(crate::core::jsonrpc::Id),
}

/// The core key error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Key {
	#[error("[core::key] invalid sub-seed, index out of bound")]
	InvalidSubSeed,
}

/// The core node error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Node {
	#[error("[core::node] failed to parse metadata")]
	ParseMetadataFailed(#[source] submetadatan::Error),
	#[error("[core::node] failed to start the node")]
	StartFailed(#[source] std::io::Error),
}

/// The core SS58 error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Ss58 {
	#[error("[core::ss58] invalid address, {address:?}")]
	InvalidAddress { address: String, source: Option<Ss58InvalidAddressSource> },
	#[error("[core::ss58] failed to calculate SS58 address")]
	CalculateSs58AddressFailed(#[source] subcryptor::Error),
}
/// The sub-error type of [`Ss58::InvalidAddress`].
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Ss58InvalidAddressSource {
	#[error("{0:?}")]
	ArrayBytes(array_bytes::Error),
	#[error(transparent)]
	Subcryptor(subcryptor::Error),
}

/// The core tokio error type.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Tokio {
	#[error(transparent)]
	OneshotRecv(tokio::sync::oneshot::error::RecvError),
	// https://github.com/tokio-rs/tokio/blob/master/tokio/src/sync/mpsc/error.rs#L12
	#[error("channel closed")]
	MpscSend,
	#[error(transparent)]
	Elapsed(tokio::time::error::Elapsed),
}
