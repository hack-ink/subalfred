#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod chain {
    use serde::Serialize;
    use serde_json::{json, Value};
    pub fn get_block_hash_once(block_number: impl Serialize) -> Value {
        crate::rpc_once(
            "chain_getBlockHash",
            ::serde_json::Value::Array(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([::serde_json::to_value(&block_number).unwrap()]),
            )),
        )
    }
    pub fn get_block_hash(id: u32, block_number: impl Serialize) -> Value {
        crate::rpc(
            id,
            "chain_getBlockHash",
            ::serde_json::Value::Array(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([::serde_json::to_value(&block_number).unwrap()]),
            )),
        )
    }
}
use serde::Serialize;
use serde_json::{json, Value};
pub fn rpc(id: u32, method: &str, params: impl Serialize) -> Value {
    ::serde_json::Value::Object({
        let mut object = ::serde_json::Map::new();
        let _ = object.insert(("jsonrpc").into(), ::serde_json::to_value(&"2.0").unwrap());
        let _ = object.insert(("id").into(), ::serde_json::to_value(&id).unwrap());
        let _ = object.insert(("method").into(), ::serde_json::to_value(&method).unwrap());
        let _ = object.insert(("params").into(), ::serde_json::to_value(&params).unwrap());
        object
    })
}
pub fn rpc_once(method: &str, params: impl Serialize) -> Value {
    crate::rpc(0, method, params)
}
