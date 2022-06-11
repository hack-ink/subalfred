// crates.io
use parity_scale_codec::Encode;
// hack-ink
use crate::core::{error, Result};

/// Substrate/Polkadot key type id.
pub struct KeyTypeId([u8; 4]);
impl KeyTypeId {
	/// Pallet's [`KeyTypeId`], aka `PalletId`, `ModuleId`.
	pub fn pallet() -> Self {
		Self(*b"modl")
	}

	/// Parachain's [`KeyTypeId`], aka `ParaId`.
	pub fn parachain() -> Self {
		Self(*b"para")
	}

	/// Similar to the `ParaId` but from the view of relaychain.
	pub fn sibling() -> Self {
		Self(*b"sibl")
	}

	/// Convert the [`KeyTypeId`] to `[u8; N]`
	pub fn to_key<const N: usize>(&self, sub_seed: &[u8]) -> Result<[u8; N]> {
		let mut result = [0; N];
		let encoded_sub_seed = sub_seed.encode();

		if (self.0.len() + encoded_sub_seed.len()) > N {
			Err(error::Key::InvalidSubSeed)?;
		}

		self.0.iter().cloned().chain(encoded_sub_seed).enumerate().for_each(|(i, v)| result[i] = v);

		Ok(result)
	}
}
