// crates.io
#[cfg(feature = "serde")] use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(rename_all = "camelCase"))]
pub struct Header<Number, Hash> {
	/// Parent hash.
	pub parent_hash: Hash,
	/// Block number.
	pub number: Number,
	/// State trie merkle root
	pub state_root: Hash,
	/// Merkle root of the extrinsics.
	pub extrinsics_root: Hash,
	// /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
	// pub digest: Digest<Hash>,
}
