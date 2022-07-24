// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn get_keys_paged_once(
	key: impl Serialize,
	count: impl Serialize,
	start_key: Option<impl Serialize>,
	at: Option<impl Serialize>,
) -> Value {
	crate::rpc_once("state_getKeysPaged", serde_json::json!([key, count, start_key, at]))
}

#[subrpcer_impl::rpc]
pub fn get_metadata_once() -> Value {
	crate::rpc_once("state_getMetadata", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn get_runtime_version_once(at: Option<impl Serialize>) -> Value {
	crate::rpc_once("state_getRuntimeVersion", serde_json::json!([at]))
}

#[subrpcer_impl::rpc]
pub fn get_storage_once(key: impl Serialize, at: Option<impl Serialize>) -> Value {
	crate::rpc_once("state_getStorage", serde_json::json!([key, at]))
}

#[subrpcer_impl::rpc]
pub fn subscribe_storage_once(storage_keys: impl Serialize) -> Value {
	crate::rpc_once("state_subscribeStorage", serde_json::json!([storage_keys]))
}

#[subrpcer_impl::rpc]
pub fn unsubscribe_storage_once(subscription_id: impl Serialize) -> Value {
	crate::rpc_once("state_unsubscribeStorage", serde_json::json!([subscription_id]))
}
