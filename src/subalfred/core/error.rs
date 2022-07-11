//! Subalfred core error collections.

use thiserror::Error as ThisError;

/// Main error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Error {
	// #[error(transparent)]
	// Simple(#[from] Simple),
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

// /// Print the error directly.
// #[derive(Debug)]
// pub struct Simple(String);
// impl std::fmt::Display for Simple {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		std::fmt::Debug::fmt(self, f)
// 	}
// }
// impl std::error::Error for Simple {}
// /// Simple debug helper.
// ///
// /// Convert the error to [`Simple`].
// pub fn quick_err<E>(e: E) -> Simple
// where
// 	E: std::fmt::Debug,
// {
// 	Simple(format!("{e:?}"))
// }

/// Cargo error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Cargo {
	#[error("[core::cargo] failed to exec `cargo metadata`")]
	ExecMetadataFailed(#[source] cargo_metadata::Error),
	#[error("[core::cargo] failed to open the manifest file")]
	OpenManifestFailed(#[source] cargo_toml::Error),
}

/// Generic error.
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
/// [`Generic::AlmostImpossible`] error helper.
pub fn almost_impossible(e_msg: &'static str) -> Generic {
	Generic::AlmostImpossible(e_msg)
}

/// JSONRPC error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Jsonrpc {
	// #[error("[core::jsonrpc] empty batch")]
	// EmptyBatch,
	#[error("[core::jsonrpc] exceeded the maximum number of request queue size, {0}")]
	ExceededRequestQueueMaxSize(crate::core::jsonrpc::Id),
}

/// Key error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Key {
	#[error("[core::key] invalid sub-seed, index out of bound")]
	InvalidSubSeed,
}

/// Node error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Node {
	#[error("[core::node] invalid specification file")]
	InvalidSpecificationFile,
	#[error("[core::node] key-values' count mismatched, expect {expect} got {got}")]
	KeyValuesCountMismatched { expect: usize, got: usize },
	#[error("[core::node] failed to parse metadata")]
	ParseMetadataFailed(#[source] submetadatan::Error),
	#[error("[core::node] failed to start the node")]
	StartNodeFailed(#[source] std::io::Error),
}

/// SS58 error..
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Ss58 {
	#[error("[core::ss58] invalid address, {address:?}")]
	InvalidAddress { address: String, source: Option<Ss58InvalidAddressSource> },
	#[error("[core::ss58] failed to calculate SS58 address")]
	CalculateSs58AddressFailed(#[source] subcryptor::Error),
}
/// Sub-error of [`Ss58::InvalidAddress`].
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Ss58InvalidAddressSource {
	#[error("{0:?}")]
	ArrayBytes(array_bytes::Error),
	#[error(transparent)]
	Subcryptor(subcryptor::Error),
}

/// Tokio error.
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
