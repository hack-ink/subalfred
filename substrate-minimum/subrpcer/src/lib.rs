pub mod author;
pub mod chain;
pub mod grandpa;
pub mod offchain;
pub mod state;
pub mod system;

#[cfg(any(feature = "isahc-client", feature = "reqwest-client"))] pub mod client;

// TODO: optimize the option param

// crates.io
use serde::Serialize;
use serde_json::{json, Value};

pub fn rpc(id: u32, method: &str, params: impl Serialize) -> Value {
	json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}

pub fn rpc_once(method: &str, params: impl Serialize) -> Value {
	crate::rpc(0, method, params)
}

#[cfg(feature = "tracing")]
pub fn debug(rpc: Value) -> Value {
	tracing::debug!("{}", rpc);

	rpc
}
