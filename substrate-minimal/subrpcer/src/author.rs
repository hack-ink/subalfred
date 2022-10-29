//! Author related methods.
//!
//! Substrate reference(s):
//! - [Author API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/client/rpc-api/src/author/mod.rs#L28-L78)

impl_apis! {
	author {
		has_key { params: [public_key, key_type], opt_params: [] }
		has_session_keys{ params: [session_keys], opt_params: [] }
		insert_key { params: [key_type, suri, public_key], opt_params: [] }
		pending_extrinsics { params: [], opt_params: [] }
		remove_extrinsic { params: [bytes_or_hash], opt_params: [] }
		rotate_keys { params: [], opt_params: [] }
		submit_and_watch_extrinsic { params: [extrinsic], opt_params: [] }
		submit_extrinsic{ params: [extrinsic], opt_params: [] }
		unwatch_extrinsic { params: [subscription_id], opt_params: [] }
	}
}
