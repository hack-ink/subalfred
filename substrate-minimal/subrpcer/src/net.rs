// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn listening_once() -> Value {
	crate::rpc_once("net_listening", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn peer_count_once() -> Value {
	crate::rpc_once("net_peerCount", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn version_once() -> Value {
	crate::rpc_once("net_version", Value::Null)
}
