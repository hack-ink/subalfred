use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error("[subcryptor] unsupported network, {0:?}")]
	UnsupportedNetwork(String),
	#[error("[subcryptor] invalid prefix, {0:?}")]
	InvalidPrefix(u8),
	#[error("[subcryptor] invalid ss58 address, {address:?}")]
	InvalidSs58Address { address: String, source: Option<InvalidSs58AddressSource> },
}
#[derive(Debug, ThisError)]
pub enum InvalidSs58AddressSource {
	#[error("{0:?}")]
	Base58(base58::FromBase58Error),
}
