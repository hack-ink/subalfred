// hack-ink
use crate::prelude::*;

#[subrpcer_impl::rpc]
pub fn query_fee_details_once(extrinsic: impl Serialize, at: Option<impl Serialize>) -> Value {
	crate::rpc_once("payment_queryFeeDetails", serde_json::json!([extrinsic, at]))
}

#[subrpcer_impl::rpc]
pub fn query_info_once(extrinsic: impl Serialize, at: Option<impl Serialize>) -> Value {
	crate::rpc_once("payment_queryInfo", serde_json::json!([extrinsic, at]))
}
