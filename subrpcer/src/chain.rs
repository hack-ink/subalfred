// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

pub fn get_block_hash_with_id(block_number: impl Serialize, id: impl Serialize) -> Value {
	rpc("chain_getBlockHash", json!([block_number]), id)
}
pub fn get_block_hash(block_number: impl Serialize) -> Value {
	get_block_hash_with_id(block_number, DEFAULT_ID)
}
#[cfg(feature = "raw-params")]
pub fn get_block_hash_with_raw_params_and_id(params: impl Serialize, id: impl Serialize) -> Value {
	rpc("chain_getBlockHash", params, id)
}
#[cfg(feature = "raw-params")]
pub fn get_block_hash_with_raw_params(params: impl Serialize) -> Value {
	get_block_hash_with_raw_params_and_id(params, DEFAULT_ID)
}
