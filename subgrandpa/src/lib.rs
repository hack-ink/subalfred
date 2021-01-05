// --- crates.io ---
#[cfg(feature = "codec")]
use parity_scale_codec::Decode;

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct GrandpaJustification<Hash, BlockNumber, Signature, AccountId> {
	pub round: u64,
	pub commit: Commit<Hash, BlockNumber, Signature, AccountId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct Commit<Hash, BlockNumber, Signature, AccountId> {
	pub target_hash: Hash,
	pub target_number: BlockNumber,
	pub precommits: Vec<SignedPrecommit<Hash, BlockNumber, Signature, AccountId>>,
}
#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct SignedPrecommit<Hash, BlockNumber, Signature, AccountId> {
	pub precommit: Precommit<Hash, BlockNumber>,
	pub signature: Signature,
	pub id: AccountId,
}
#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct Precommit<Hash, BlockNumber> {
	pub target_hash: Hash,
	pub target_number: BlockNumber,
}
