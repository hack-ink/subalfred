// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn account_next_index_once(account_id: impl Serialize) -> Value {
	crate::rpc_once("system_accountNextIndex", serde_json::json!([account_id]))
}

#[subrpcer_impl::rpc]
pub fn add_log_filter_once(directives: impl Serialize) -> Value {
	crate::rpc_once("system_addLogFilter", serde_json::json!([directives]))
}

#[subrpcer_impl::rpc]
pub fn add_reserved_peer_once(peer: impl Serialize) -> Value {
	crate::rpc_once("system_addReservedPeer", serde_json::json!([peer]))
}

#[subrpcer_impl::rpc]
pub fn chain_once() -> Value {
	crate::rpc_once("system_chain", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn chain_type_once() -> Value {
	crate::rpc_once("system_chainType", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn dry_run_once(extrinsic: impl Serialize, at: Option<impl Serialize>) -> Value {
	crate::rpc_once("system_dryRun", serde_json::json!([extrinsic, at]))
}

#[subrpcer_impl::rpc]
pub fn health_once() -> Value {
	crate::rpc_once("system_health", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn local_listen_addresses_once() -> Value {
	crate::rpc_once("system_localListenAddresses", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn local_peer_id_once() -> Value {
	crate::rpc_once("system_localPeerId", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn name_once() -> Value {
	crate::rpc_once("system_name", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn node_roles_once() -> Value {
	crate::rpc_once("system_nodeRoles", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn peers_once() -> Value {
	crate::rpc_once("system_peers", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn properties_once() -> Value {
	crate::rpc_once("system_properties", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn remove_reserved_peer_once(peer_id: impl Serialize) -> Value {
	crate::rpc_once("system_removeReservedPeer", serde_json::json!([peer_id]))
}

#[subrpcer_impl::rpc]
pub fn reserved_peers_once() -> Value {
	crate::rpc_once("system_reservedPeers", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn reset_log_filter_once() -> Value {
	crate::rpc_once("system_resetLogFilter", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn sync_state_once() -> Value {
	crate::rpc_once("system_syncState", Value::Null)
}

#[subrpcer_impl::rpc]
pub fn version_once() -> Value {
	crate::rpc_once("system_version", Value::Null)
}
