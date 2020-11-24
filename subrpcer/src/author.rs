// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::rpc;

pub fn submit_and_watch_extrinsic(extrinsic: impl Serialize) -> Value {
	rpc("author_submitAndWatchExtrinsic", json!([extrinsic]), 3)
}
