// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn epoch_authorship_once() -> Value {
	crate::rpc_once("babe_epochAuthorship", Value::Null)
}
