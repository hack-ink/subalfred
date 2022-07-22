// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn local_storage_get_once(kind: impl Serialize, key: impl Serialize) -> Value {
	crate::rpc_once("offchain_localStorageGet", serde_json::json!([kind, key]))
}

#[subrpcer_impl::rpc]
pub fn local_storage_set_once(
	kind: impl Serialize,
	key: impl Serialize,
	value: impl Serialize,
) -> Value {
	crate::rpc_once("offchain_localStorageSet", serde_json::json!([kind, key, value]))
}
