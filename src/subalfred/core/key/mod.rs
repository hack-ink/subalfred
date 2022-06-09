// crates.io
use parity_scale_codec::Encode;
// hack-ink
use crate::core::{error, Result};

pub struct KeyTypeId([u8; 4]);
impl KeyTypeId {
	pub fn pallet() -> Self {
		Self(*b"modl")
	}

	pub fn parachain() -> Self {
		Self(*b"para")
	}

	pub fn sibling() -> Self {
		Self(*b"sibl")
	}

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
