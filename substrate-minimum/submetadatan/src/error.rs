use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error("[submetadatan] unsupported version, {0:?}")]
	UnsupportedVersion(u32),
}
