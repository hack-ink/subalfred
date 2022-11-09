//! Net related methods.
//!
//! Frontier reference(s):
//! - [Net API(s)](https://github.com/paritytech/frontier/blob/eef5723675850166da904b295b7dfa90894b1270/client/rpc-core/src/net.rs#L25-L40)

impl_apis! {
	net {
		listening { params: [], opt_params: [] }
		peer_count { params: [], opt_params: [] }
		version { params: [], opt_params: [] }
	}
}
