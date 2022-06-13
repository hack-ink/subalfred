pub mod author;
pub mod babe;
pub mod chain;
pub mod grandpa;
pub mod net;
pub mod offchain;
pub mod payment;
pub mod rpc;
pub mod state;
pub mod system;

#[cfg(any(feature = "isahc-client", feature = "reqwest-client"))] pub mod client;

mod prelude {
	pub use serde::Serialize;
	pub use serde_json::Value;
}

// TODO: optimize the option param

// crates.io
use prelude::*;

pub fn rpc(id: usize, method: &str, params: Value) -> Value {
	serde_json::json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}

pub fn rpc_once(method: &str, params: Value) -> Value {
	crate::rpc(0, method, params)
}

#[cfg(feature = "tracing")]
pub fn debug(rpc: Value) -> Value {
	tracing::debug!("{}", rpc);

	rpc
}
