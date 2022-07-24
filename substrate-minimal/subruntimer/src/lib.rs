// crates.io
#[cfg(feature = "serde")] use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(rename_all = "camelCase"))]
pub struct Header<Number, Hash> {
	/// The parent hash.
	pub parent_hash: Hash,
	/// The block number.
	pub number: Number,
	/// The state trie merkle root
	pub state_root: Hash,
	/// The merkle root of the extrinsics.
	pub extrinsics_root: Hash,
	// /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
	// pub digest: Digest<Hash>,
}
