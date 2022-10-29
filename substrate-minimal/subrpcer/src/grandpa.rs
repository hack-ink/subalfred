//! GRANDPA related methods.
//!
//! Substrate reference(s):
//! - [GRANDPA API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/client/finality-grandpa/rpc/src/lib.rs#L46-L67)

impl_apis! {
	grandpa {
		prove_finality { params: [block_number], opt_params: [] }
		round_state { params: [], opt_params: [] }
		subscribe_justifications { params: [], opt_params: [] }
		unsubscribe_justifications { params: [subscription_id], opt_params: [] }
	}
}
