// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

pub fn properties_with_id(id: impl Serialize) -> Value {
	rpc("system_properties", Value::Null, id)
}
pub fn properties() -> Value {
	get_metadata_with_id(DEFAULT_ID)
}
