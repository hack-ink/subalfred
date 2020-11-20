// --- crates.io ---
use serde_json::{json, Value};
// --- subrpcer ---
use crate::rpc;

pub fn get_runtime_version() -> Value {
	rpc("state_getRuntimeVersion", json!([]))
}
