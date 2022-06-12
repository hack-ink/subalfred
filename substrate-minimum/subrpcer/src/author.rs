// crates.io
use serde::Serialize;
use serde_json::{json, Value};

#[subrpcer_impl::rpc]
pub fn submit_and_watch_extrinsic_once(extrinsic: impl Serialize) -> Value {
	crate::rpc_once("author_submitAndWatchExtrinsic", json!([extrinsic]))
}

#[subrpcer_impl::rpc]
pub fn unwatch_extrinsic_once(subscription_id: impl Serialize) -> Value {
	crate::rpc_once("author_unwatchExtrinsic", json!([subscription_id]))
}
