/// Error library.
pub mod error;
pub use error::Error;

/// The core library about interacting with Cargo.
pub mod cargo;
/// The core library about interacting with HTTP.
pub mod http;
/// The core library about interacting with JSONRPC.
pub mod jsonrpc;
/// The core library about interacting with Substrate-based node.
pub mod node;
/// The core library about interacting with OS.
pub mod system;
/// The core library about interacting with websocket.
pub mod websocket;

/// Substrate checkers library.
pub mod check;
/// Substrate key library.
pub mod key;
/// SS58 address library.
pub mod ss58;

/// Subalfred core lib's `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
