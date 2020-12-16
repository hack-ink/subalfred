// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

pub fn submit_and_watch_extrinsic_with_id(extrinsic: impl Serialize, id: impl Serialize) -> Value {
	rpc("author_submitAndWatchExtrinsic", json!([extrinsic]), id)
}
pub fn submit_and_watch_extrinsic(extrinsic: impl Serialize) -> Value {
	submit_and_watch_extrinsic_with_id(extrinsic, DEFAULT_ID)
}
#[cfg(feature = "raw-params")]
pub fn submit_and_watch_extrinsic_with_params_and_id(
	params: impl Serialize,
	id: impl Serialize,
) -> Value {
	rpc("author_submitAndWatchExtrinsic", params, id)
}
#[cfg(feature = "raw-params")]
pub fn submit_and_watch_extrinsic_with_params(params: impl Serialize) -> Value {
	submit_and_watch_extrinsic_with_params_and_id(params, DEFAULT_ID)
}

pub fn unwatch_extrinsic_with_id(subscription_id: impl Serialize, id: impl Serialize) -> Value {
	rpc("author_unwatchExtrinsic", json!([subscription_id]), id)
}
pub fn unwatch_extrinsic(subscription_id: impl Serialize) -> Value {
	unwatch_extrinsic_with_id(subscription_id, DEFAULT_ID)
}
#[cfg(feature = "raw-params")]
pub fn unwatch_extrinsic_with_params_and_id(params: impl Serialize, id: impl Serialize) -> Value {
	rpc("author_unwatchExtrinsic", params, id)
}
#[cfg(feature = "raw-params")]
pub fn unwatch_extrinsic_with_params(params: impl Serialize) -> Value {
	unwatch_extrinsic_with_params_and_id(params, DEFAULT_ID)
}
