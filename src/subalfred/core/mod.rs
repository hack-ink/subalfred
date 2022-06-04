pub mod error;
pub use error::Error;

pub mod cargo;
pub mod check;
pub mod future;
pub mod http;
pub mod node;
pub mod ss58;
pub mod xcm;

pub type Result<T> = ::std::result::Result<T, Error>;
