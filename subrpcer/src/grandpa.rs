// crates.io
use serde::Serialize;
use serde_json::Value;
// hack-ink
use crate::{rpc, DEFAULT_ID};

#[subrpcer_impl::rpc]
pub fn round_state() -> Value {
	rpc(DEFAULT_ID, "grandpa_roundState", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn subscribe_justifications() -> Value {
	rpc(DEFAULT_ID, "grandpa_subscribeJustifications", Value::Null)
}
