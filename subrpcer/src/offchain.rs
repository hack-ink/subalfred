// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

#[subrpcer_impl::rpc]
pub fn local_storage_get(kind: impl Serialize, key: impl Serialize) -> Value {
	rpc(DEFAULT_ID, "offchain_localStorageGet", json!([kind, key]))
}

#[subrpcer_impl::rpc]
pub fn local_storage_set(
	kind: impl Serialize,
	key: impl Serialize,
	value: impl Serialize,
) -> Value {
	rpc(
		DEFAULT_ID,
		"offchain_localStorageSet",
		json!([kind, key, value]),
	)
}
