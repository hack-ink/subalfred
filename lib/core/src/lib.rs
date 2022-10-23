//! Subalfred core libraries.

pub mod cargo;
pub mod check;
pub mod error;
pub mod http;
pub mod jsonrpc;
pub mod key;
pub mod node;
pub mod ss58;
pub mod state;
pub mod substrate_client;
pub mod system;

pub mod prelude {
	//! Subalfred core libraries prelude.

	pub use ::std::result::Result as StdResult;

	pub use super::error::{self, Error};

	/// Subalfred core lib's `Result` type.
	pub type Result<T> = ::std::result::Result<T, Error>;
}
