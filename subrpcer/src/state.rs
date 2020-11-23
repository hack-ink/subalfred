// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::rpc;

pub fn get_metadata() -> Value {
	rpc("state_getMetadata", Value::Null)
}

pub fn get_runtime_version() -> Value {
	rpc("state_getRuntimeVersion", Value::Null)
}

pub fn get_storage(key: impl Serialize, at: Option<impl Serialize>) -> Value {
	rpc(
		"state_getStorage",
		json!([
			key,
			at.map(|at| serde_json::to_value(at).unwrap())
				.unwrap_or(Value::Null)
		]),
	)
}
