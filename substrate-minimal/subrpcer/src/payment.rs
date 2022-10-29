//! Payment related methods.
//!
//! Substrate reference(s):
//! - [Payment API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/frame/transaction-payment/rpc/src/lib.rs#L40-L51)

impl_apis! {
	payment {
		query_info { params: [encoded_tx], opt_params: [at] }
		query_fee_details { params: [encoded_tx], opt_params: [at] }
	}
}
