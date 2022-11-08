//! Minimal implementation of Substrate RPC APIs.
//!
//! Formatting rule(s):
//! - All the APIs must be sorted in alphabetic order.

#![deny(missing_docs)]

// TODO: https://github.com/rust-lang/rust/issues/82715
// This should be `no_run` eventually
/// Define a group of APIs.
///
/// Require [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) as the dependencies.
///
/// # Example
/// ```ignore
/// impl_apis! {
/// 	state {
/// 		call { params: [name, bytes], opt_params: [hash] }
/// 		get_keys { params: [prefix], opt_params: [hash] }
/// 		get_metadata { params: [], opt_params: [hash] }
/// 		get_pairs { params: [prefix], opt_params: [hash] }
/// 		get_read_proof { params: [keys], opt_params: [hash] }
/// 		get_runtime_version { params: [], opt_params: [hash] }
/// 		get_storage { params: [key], opt_params: [hash] }
/// 		get_storage_hash { params: [key], opt_params: [hash] }
/// 		get_storage_size { params: [key], opt_params: [hash] }
/// 		query_storage { params: [keys, block], opt_params: [hash] }
/// 		query_storage_at { params: [keys], opt_params: [at] }
/// 		subscribe_runtime_version { params: [], opt_params: [] }
/// 		subscribe_storage { params: [], opt_params: [keys] }
/// 		trace_block { params: [block], opt_params: [targets, storage_keys, methods] }
/// 		unsubscribe_runtime_version { params: [subscription_id], opt_params: [] }
/// 		unsubscribe_storage { params: [subscription_id], opt_params: [] }
/// 	}
/// }
/// ```
#[allow(clippy::tabs_in_doc_comments)]
#[macro_export]
macro_rules! impl_apis {
	{
		$prefix:ident {
			$(
				$method:ident {
					params: [
						$(
							$param:ident
						),*
					],
					opt_params: [
						$(
							$opt_param:ident
						),*
					]
				}
			)+
		}
	} => {
		$(
			affix::paste! {
				/// Check module's Substrate reference(s) for the detail.
				pub fn $method(
					id: usize,
					$($param: impl serde::Serialize,)*
					$($opt_param: Option<impl serde::Serialize>,)*
				) -> serde_json::Value {
					$crate::rpc(
						id,
						stringify!([<$prefix _ $method:camel>]),
						serde_json::json!([
							$($param,)*
							$($opt_param,)*
						])
					)
				}

				#[doc = concat!("Similar to [`", stringify!($method), "`], but return the method name and parameters directly.")]
				pub fn [<$method _raw>](
					$($param: impl serde::Serialize,)*
					$($opt_param: Option<impl serde::Serialize>,)*
				) -> (&'static str, serde_json::Value) {
					(
						stringify!([<$prefix _ $method:camel>]),
						serde_json::json!([
							$($param,)*
							$($opt_param,)*
						])
					)
				}
			}
		)+
	};
}

pub mod author;
pub mod babe;
pub mod chain;
pub mod grandpa;
pub mod net;
pub mod offchain;
pub mod payment;
pub mod rpc;
pub mod state;
pub mod system;

#[cfg(any(feature = "reqwest-client", feature = "ureq-client"))] mod client;

// TODO: optimize the option param

// crates.io
use serde_json::Value;

/// Build a JSONRPC 2.0 call.
pub fn rpc(id: usize, method: &str, params: Value) -> Value {
	serde_json::json!({
		"jsonrpc": "2.0",
		"id": id,
		"method": method,
		"params": params
	})
}

/// A debug wrapper for the RPC payload.
///
/// This will output a trace-level log about the RPC call detail.
#[cfg(feature = "tracing")]
pub fn debug(rpc: Value) -> Value {
	tracing::trace!("{rpc:?}");

	rpc
}
