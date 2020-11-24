pub mod author;
pub mod chain;
pub mod state;

// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};

pub fn rpc(method: impl Serialize, params: impl Serialize, id: impl Serialize) -> Value {
	json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}
