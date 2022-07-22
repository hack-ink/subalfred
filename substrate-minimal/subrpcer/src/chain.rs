// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn get_block_once(hash: impl Serialize) -> Value {
	crate::rpc_once("chain_getBlock", serde_json::json!([hash]))
}

#[subrpcer_impl::rpc]
pub fn get_block_hash_once(block_number: impl Serialize) -> Value {
	crate::rpc_once("chain_getBlockHash", serde_json::json!([block_number]))
}

#[subrpcer_impl::rpc]
pub fn get_finalized_head_once() -> Value {
	crate::rpc_once("chain_getFinalizedHead", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn get_header_once(hash: impl Serialize) -> Value {
	crate::rpc_once("chain_getHeader", serde_json::json!([hash]))
}
