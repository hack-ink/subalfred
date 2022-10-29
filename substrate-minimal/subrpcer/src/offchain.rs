//! Offchain related methods.
//!
//! Substrate reference(s):
//! - [Offchain API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/client/rpc-api/src/offchain/mod.rs#L26-L36)

impl_apis! {
	offchain {
		local_storage_get { params: [kind, key], opt_params: [] }
		local_storage_set { params: [kind, key, value], opt_params: [] }
	}
}
