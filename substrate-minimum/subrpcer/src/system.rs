// crates.io
use serde_json::Value;

#[subrpcer_impl::rpc]
pub fn chain_once() -> Value {
	crate::rpc_once("system_chain", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn chain_type_once() -> Value {
	crate::rpc_once("system_chainType", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn properties_once() -> Value {
	crate::rpc_once("system_properties", Value::Null)
}
