pub mod error;
pub use error::Error;

pub mod cargo;
pub mod check;
pub mod http;
pub mod key;
pub mod node;
pub mod ss58;
pub mod system;

pub type Result<T> = ::std::result::Result<T, Error>;
