// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::rpc;

pub fn get_block_hash(block_number: impl Serialize) -> Value {
	rpc("chain_getBlockHash", json!([block_number]), 1)
}
#[cfg(feature = "raw-params")]
pub fn get_block_hash_with_raw_params(params: impl Serialize) -> Value {
	rpc("chain_getBlockHash", params, 1)
}
