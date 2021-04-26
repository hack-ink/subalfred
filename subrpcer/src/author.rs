// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

#[subrpcer_impl::rpc]
pub fn submit_and_watch_extrinsic(extrinsic: impl Serialize) -> Value {
	rpc(
		DEFAULT_ID,
		"author_submitAndWatchExtrinsic",
		json!([extrinsic]),
	)
}

#[subrpcer_impl::rpc]
pub fn unwatch_extrinsic(subscription_id: impl Serialize) -> Value {
	rpc(
		DEFAULT_ID,
		"author_unwatchExtrinsic",
		json!([subscription_id]),
	)
}
