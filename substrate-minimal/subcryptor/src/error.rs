use thiserror::Error as ThisError;

/// Main error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Error {
	#[error("[subcryptor] from base58 error, {0:?}")]
    FromBase58(base58::FromBase58Error),
	#[error("[subcryptor] invalid prefix, {0:?}")]
	InvalidPrefix(u8),
	#[error("[subcryptor] invalid ss58 address, {0:?}")]
	InvalidSs58Address(String),
	#[error("[subcryptor] unsupported network, {0:?}")]
	UnsupportedNetwork(String),
}
