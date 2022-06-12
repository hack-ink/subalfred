// crates.io
use serde::Serialize;
use serde_json::{json, Value};

#[subrpcer_impl::rpc]
pub fn get_block_hash_once(block_number: impl Serialize) -> Value {
	crate::rpc_once("chain_getBlockHash", json!([block_number]))
}
