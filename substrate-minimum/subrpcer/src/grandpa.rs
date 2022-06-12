// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn proof_finality_once(block_number: impl Serialize) -> Value {
	crate::rpc_once("grandpa_proofFinality", serde_json::json!([block_number]))
}

#[subrpcer_impl::rpc]
pub fn round_state_once() -> Value {
	crate::rpc_once("grandpa_roundState", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn subscribe_justifications_once() -> Value {
	crate::rpc_once("grandpa_subscribeJustifications", Value::Null)
}
