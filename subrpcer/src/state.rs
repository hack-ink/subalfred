// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- hack-ink ---
use crate::{rpc, DEFAULT_ID};

#[subrpcer_impl::rpc]
pub fn get_metadata() -> Value {
	rpc(DEFAULT_ID, "state_getMetadata", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn get_runtime_version() -> Value {
	rpc(DEFAULT_ID, "state_getRuntimeVersion", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn get_storage(key: impl Serialize, at: Option<impl Serialize>) -> Value {
	rpc(
		DEFAULT_ID,
		"state_getStorage",
		json!([
			key,
			at.map(|at| serde_json::to_value(at).unwrap())
				.unwrap_or(Value::Null)
		]),
	)
}

#[subrpcer_impl::rpc]
pub fn subscribe_storage(storage_keys: impl Serialize) -> Value {
	rpc(DEFAULT_ID, "state_subscribeStorage", json!([storage_keys]))
}

#[subrpcer_impl::rpc]
pub fn unsubscribe_storage(subscription_id: impl Serialize) -> Value {
	rpc(
		DEFAULT_ID,
		"state_unsubscribeStorage",
		json!([subscription_id]),
	)
}
