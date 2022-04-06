pub mod author;
pub mod chain;
pub mod grandpa;
pub mod offchain;
pub mod state;
pub mod system;

#[cfg(any(feature = "isahc-client", feature = "ureq-client"))]
pub mod client;

// crates.io
use serde::Serialize;
use serde_json::{json, Value};

const DEFAULT_ID: u8 = 1;

pub fn rpc(id: impl Serialize, method: impl Serialize, params: impl Serialize) -> Value {
	json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}

#[cfg(feature = "tracing")]
pub fn debug_rpc(rpc: Value) -> Value {
	tracing::debug!("{}", rpc);

	rpc
}
