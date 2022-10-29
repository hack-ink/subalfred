use thiserror::Error as ThisError;

/// Main error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Error {
	#[error("[submetadatan] unsupported version, {0:?}")]
	UnsupportedVersion(u32),
}
