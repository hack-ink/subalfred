//! Subalfred's core libraries.

#![deny(missing_docs, unused_crate_dependencies)]

pub mod prelude {
	//! Subalfred's core prelude.

	pub use std::result::Result as StdResult;

	pub use super::error::{self, Error};

	/// Subalfred's core libraries' [`Result`] type.
	pub type Result<T> = StdResult<T, Error>;
}

pub mod cargo;
pub mod check;
pub mod error;
pub mod github;
pub mod http;
pub mod jsonrpc;
pub mod key;
pub mod node;
pub mod ss58;
pub mod state;
pub mod substrate_client;
pub mod system;
