// crates.io
use serde::Serialize;
use serde_json::{json, Value};

#[subrpcer_impl::rpc]
pub fn local_storage_get_once(kind: impl Serialize, key: impl Serialize) -> Value {
	crate::rpc_once("offchain_localStorageGet", json!([kind, key]))
}

#[subrpcer_impl::rpc]
pub fn local_storage_set_once(
	kind: impl Serialize,
	key: impl Serialize,
	value: impl Serialize,
) -> Value {
	crate::rpc_once("offchain_localStorageSet", json!([kind, key, value]))
}
