// --- crates.io ---
use serde_json::Value;
// --- subrpcer ---
use crate::rpc;

pub fn get_metadata() -> Value {
	rpc("state_getMetadata", Value::Null)
}

pub fn get_runtime_version() -> Value {
	rpc("state_getRuntimeVersion", Value::Null)
}
