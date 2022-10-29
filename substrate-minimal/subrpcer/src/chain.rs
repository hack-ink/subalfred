//! Chain related methods.
//!
//! Substrate reference(s):
//! - [Chain API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/client/rpc-api/src/chain/mod.rs#L26-L76)

impl_apis! {
	chain {
		get_block { params: [], opt_params: [hash] }
		get_block_hash { params: [], opt_params: [list_or_value] }
		get_finalized_head { params: [], opt_params: [] }
		get_header { params: [], opt_params: [hash] }
		subscribe_new_heads { params: [], opt_params: [] }
		unsubscribe_new_heads { params: [subscription_id], opt_params: [] }
		subscribe_finalized_heads { params: [], opt_params: [] }
		unsubscribe_finalized_heads { params: [subscription_id], opt_params: [] }
	}
}
