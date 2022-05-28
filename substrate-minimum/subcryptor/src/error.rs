use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error("[subcryptor] unsupported network, {0:?}")]
	UnsupportedNetwork(String),
	#[error("[subcryptor] invalid prefix, {0:?}")]
	InvalidPrefix(u8),
	#[error("[subcryptor] invalid ss58 address, {0:?}")]
	InvalidSs58Address(base58::FromBase58Error),
}
