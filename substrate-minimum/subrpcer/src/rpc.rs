// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn methods_once() -> Value {
	crate::rpc_once("rpc_methods", Value::Null)
}
