#[cfg(feature = "isahc-client")]
pub mod i;
#[cfg(feature = "ureq-client")]
pub mod u;

// crates.io
use serde_json::Error as SerdeJsonError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[error("{0}")]
pub enum Error {
	SerdeJson(#[from] SerdeJsonError),
	#[cfg(feature = "isahc-client")]
	Isahc(#[from] i::IsahcError),
	#[cfg(feature = "ureq-client")]
	Ureq(#[from] u::UreqError),
}
