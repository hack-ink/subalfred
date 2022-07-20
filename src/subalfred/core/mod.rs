//! Subalfred core libraries.

pub mod error;

pub mod cargo;
pub mod http;
pub mod jsonrpc;
pub mod node;
pub mod substrate_client;
pub mod system;

pub mod check;
pub mod key;
pub mod ss58;

pub mod prelude {
	//! Subalfred core libraries prelude.

	pub use ::std::result::Result as StdResult;

	pub use super::error::{self, Error};

	/// Subalfred core lib's `Result` type.
	pub type Result<T> = ::std::result::Result<T, Error>;
}
