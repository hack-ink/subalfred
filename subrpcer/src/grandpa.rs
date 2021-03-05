// --- crates.io ---
use serde::Serialize;
use serde_json::Value;
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

pub fn round_state_with_id(id: impl Serialize) -> Value {
	rpc("grandpa_roundState", Value::Null, id)
}
pub fn round_state() -> Value {
	round_state_with_id(DEFAULT_ID)
}

pub fn subscribe_justifications_with_id(id: impl Serialize) -> Value {
	rpc("grandpa_subscribeJustifications", Value::Null, id)
}
pub fn subscribe_justifications() -> Value {
	subscribe_justifications_with_id(DEFAULT_ID)
}
