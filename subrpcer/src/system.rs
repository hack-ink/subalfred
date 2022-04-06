// crates.io
use serde::Serialize;
use serde_json::Value;
// hack-ink
use crate::{rpc, DEFAULT_ID};

#[subrpcer_impl::rpc]
pub fn properties() -> Value {
	rpc(DEFAULT_ID, "system_properties", Value::Null)
}
