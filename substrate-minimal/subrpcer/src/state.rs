//! State related methods.
//!
//! Substrate reference(s):
//! - [State API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/client/rpc-api/src/state/mod.rs#L33-L288)

impl_apis! {
	state {
		call { params: [name, bytes], opt_params: [hash] }
		get_keys { params: [prefix], opt_params: [hash] }
		get_keys_paged { params: [count], opt_params: [prefix, start_key, hash] }
		get_metadata { params: [], opt_params: [hash] }
		get_pairs { params: [prefix], opt_params: [hash] }
		get_read_proof { params: [keys], opt_params: [hash] }
		get_runtime_version { params: [], opt_params: [hash] }
		get_storage { params: [key], opt_params: [hash] }
		get_storage_hash { params: [key], opt_params: [hash] }
		get_storage_size { params: [key], opt_params: [hash] }
		query_storage { params: [keys, block], opt_params: [hash] }
		query_storage_at { params: [keys], opt_params: [at] }
		subscribe_runtime_version { params: [], opt_params: [] }
		subscribe_storage { params: [], opt_params: [keys] }
		trace_block { params: [block], opt_params: [targets, storage_keys, methods] }
		unsubscribe_runtime_version { params: [subscription_id], opt_params: [] }
		unsubscribe_storage { params: [subscription_id], opt_params: [] }
	}
}
