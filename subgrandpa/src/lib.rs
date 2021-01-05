// --- crates.io ---
#[cfg(feature = "codec")]
use parity_scale_codec::Decode;

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
struct GrandpaJustification<Hash, BlockNumber, Signature, AccountId> {
	round: u64,
	commit: Commit<Hash, BlockNumber, Signature, AccountId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
struct Commit<Hash, BlockNumber, Signature, AccountId> {
	target_hash: Hash,
	target_number: BlockNumber,
	precommits: Vec<SignedPrecommit<Hash, BlockNumber, Signature, AccountId>>,
}
#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
struct SignedPrecommit<Hash, BlockNumber, Signature, AccountId> {
	precommit: Precommit<Hash, BlockNumber>,
	signature: Signature,
	id: AccountId,
}
#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
struct Precommit<Hash, BlockNumber> {
	target_hash: Hash,
	target_number: BlockNumber,
}
