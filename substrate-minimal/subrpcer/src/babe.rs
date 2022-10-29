//! BABE related methods.
//!
//! Substrate reference(s):
//! - [BABE API(s)](https://github.com/paritytech/substrate/blob/be259234bfee056bef970ac372e04a74411c5224/client/consensus/babe/rpc/src/lib.rs#L44-L51)

impl_apis! {
	babe {
		epoch_authorship { params: [], opt_params: [] }
	}
}
