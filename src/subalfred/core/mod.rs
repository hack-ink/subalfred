//! Subalfred core libraries.

pub mod error;
pub use error::Error;

pub mod cargo;
pub mod http;
pub mod jsonrpc;
pub mod node;
pub mod substrate_client;
pub mod system;

pub mod check;
pub mod key;
pub mod ss58;

/// Subalfred core lib's `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
