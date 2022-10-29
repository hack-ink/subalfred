//! System related methods.
//!
//! Substrate reference(s):
//! - [System API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/client/rpc-api/src/system/mod.rs#L31-L123)

impl_apis! {
	system {
		add_log_filter { params: [directives], opt_params: [] }
		add_reserved_peer { params: [], opt_params: [] }
		chain { params: [], opt_params: [] }
		health { params: [], opt_params: [] }
		local_listen_addresses { params: [], opt_params: [] }
		local_peer_id { params: [], opt_params: [] }
		name { params: [], opt_params: [] }
		network_state { params: [], opt_params: [] }
		node_roles { params: [], opt_params: [] }
		peers { params: [], opt_params: [] }
		properties { params: [], opt_params: [] }
		remove_reserved_peer { params: [], opt_params: [] }
		reserved_peers { params: [], opt_params: [] }
		reset_log_filter { params: [], opt_params: [] }
		sync_state { params: [], opt_params: [] }
		version { params: [], opt_params: [] }
	}
}

// TODO: because stringify!(r#type) -> "r#type", this will break the macro rules.
/// Check module's Substrate reference(s) for the detail.
pub fn r#type(id: usize) -> serde_json::Value {
	crate::rpc(id, "system_type", serde_json::json!({}))
}
/// Similar to [type()], but return the method name and parameters directly.
pub fn type_raw() -> (&'static str, serde_json::Value) {
	("system_type", serde_json::json!({}))
}
