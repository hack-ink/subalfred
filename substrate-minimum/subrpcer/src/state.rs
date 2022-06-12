// crates.io
use serde::Serialize;
use serde_json::{json, Value};

#[subrpcer_impl::rpc]
pub fn get_keys_paged_once(
	key: impl Serialize,
	count: impl Serialize,
	start_key: Option<impl Serialize>,
	at: Option<impl Serialize>,
) -> Value {
	crate::rpc_once("state_getKeysPaged", json!([key, count, start_key, at]))
}

#[subrpcer_impl::rpc]
pub fn get_metadata_once() -> Value {
	crate::rpc_once("state_getMetadata", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn get_runtime_version_once() -> Value {
	crate::rpc_once("state_getRuntimeVersion", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn get_storage_once(key: impl Serialize, at: Option<impl Serialize>) -> Value {
	crate::rpc_once("state_getStorage", json!([key, at]))
}

#[subrpcer_impl::rpc]
pub fn subscribe_storage_once(storage_keys: impl Serialize) -> Value {
	crate::rpc_once("state_subscribeStorage", json!([storage_keys]))
}

#[subrpcer_impl::rpc]
pub fn unsubscribe_storage_once(subscription_id: impl Serialize) -> Value {
	crate::rpc_once("state_unsubscribeStorage", json!([subscription_id]))
}
