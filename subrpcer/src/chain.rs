// --- crates.io ---
use serde::Serialize;
use serde_json::{json, Value};
// --- subrpcer ---
use crate::{rpc, DEFAULT_ID};

#[subrpcer_impl::rpc]
pub fn get_block_hash(block_number: impl Serialize) -> Value {
	rpc(DEFAULT_ID, "chain_getBlockHash", json!([block_number]))
}
