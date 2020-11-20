pub mod chain;
pub mod state;

// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};

pub fn rpc(method: impl Serialize, params: impl Serialize) -> Value {
	json!({
		"jsonrpc": "2.0",
		"id": 1,
		"method": method,
		"params": params
	})
}
