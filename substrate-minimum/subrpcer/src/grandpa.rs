// crates.io
use serde_json::Value;

#[subrpcer_impl::rpc]
pub fn round_state_once() -> Value {
	crate::rpc_once("grandpa_roundState", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn subscribe_justifications_once() -> Value {
	crate::rpc_once("grandpa_subscribeJustifications", Value::Null)
}
