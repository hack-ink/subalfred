// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

pub fn subscribe_justifications_with_id(id: impl Serialize) -> Value {
	rpc("grandpa_subscribeJustifications", Value::Null, id)
}
pub fn subscribe_justifications() -> Value {
	subscribe_justifications_with_id(DEFAULT_ID)
}
