// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn has_key_once(public_key: impl Serialize, key_type: impl Serialize) -> Value {
	crate::rpc_once("author_hasKey", serde_json::json!([public_key, key_type]))
}

#[subrpcer_impl::rpc]
pub fn has_session_keys_once(session_keys: impl Serialize) -> Value {
	crate::rpc_once("author_hasSessionKeys", serde_json::json!([session_keys]))
}

#[subrpcer_impl::rpc]
pub fn insert_key_once(
	key_type: impl Serialize,
	suri: impl Serialize,
	public_key: impl Serialize,
) -> Value {
	crate::rpc_once("author_insertKey", serde_json::json!([key_type, suri, public_key]))
}

#[subrpcer_impl::rpc]
pub fn pending_extrinsics_once() -> Value {
	crate::rpc_once("author_pendingExtrinsics", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn remove_extrinsic_once(bytes_or_hash: impl Serialize) -> Value {
	crate::rpc_once("author_removeExtrinsic", serde_json::json!([bytes_or_hash]))
}

#[subrpcer_impl::rpc]
pub fn rotate_keys_once() -> Value {
	crate::rpc_once("author_rotateKeys", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn submit_extrinsic_once(extrinsic: impl Serialize) -> Value {
	crate::rpc_once("author_submitExtrinsic", serde_json::json!([extrinsic]))
}

#[subrpcer_impl::rpc]
pub fn submit_and_watch_extrinsic_once(extrinsic: impl Serialize) -> Value {
	crate::rpc_once("author_submitAndWatchExtrinsic", serde_json::json!([extrinsic]))
}

#[subrpcer_impl::rpc]
pub fn unwatch_extrinsic_once(subscription_id: impl Serialize) -> Value {
	crate::rpc_once("author_unwatchExtrinsic", serde_json::json!([subscription_id]))
}
