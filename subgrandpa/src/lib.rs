// --- crates.io ---
#[cfg(feature = "codec")]
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct GrandpaJustification<Hash, BlockNumber, Signature, AccountId> {
	pub round: u64,
	pub commit: Commit<Hash, BlockNumber, Signature, AccountId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct Commit<Hash, BlockNumber, Signature, AccountId> {
	pub target_hash: Hash,
	pub target_number: BlockNumber,
	pub precommits: Vec<SignedPrecommit<Hash, BlockNumber, Signature, AccountId>>,
}
#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct SignedPrecommit<Hash, BlockNumber, Signature, AccountId> {
	pub precommit: Precommit<Hash, BlockNumber>,
	pub signature: Signature,
	pub id: AccountId,
}
#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct Precommit<Hash, BlockNumber> {
	pub target_hash: Hash,
	pub target_number: BlockNumber,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RoundState<AccountId> {
	pub round: u32,
	pub total_weight: u32,
	pub threshold_weight: u32,
	pub prevotes: Prevotes<AccountId>,
	pub precommits: Precommits<AccountId>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Prevotes<AccountId> {
	pub current_weight: u32,
	pub missing: Vec<AccountId>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Precommits<AccountId> {
	pub current_weight: u32,
	pub missing: Vec<AccountId>,
}
