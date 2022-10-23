//! Substrate keys implementations.

// std
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	mem,
};
// crates.io
use parity_scale_codec::{Decode, Encode};
// hack-ink
use crate::prelude::*;

/// Substrate/Polkadot keys like.
pub trait Key
where
	Self: Sized,
{
	///  Key Id.
	const ID: [u8; 4];

	/// Sub-seed type.
	type SubSeed: Encode;

	/// Sub-seed, used to derive the key.
	fn sub_seed(self) -> Self::SubSeed;

	/// Convert the [`KeyTypeId`] to `[u8; N]`
	fn to_key<const N: usize>(self) -> Result<[u8; N]> {
		let mut result = [0; N];
		let sub_seed = self.sub_seed();
		let encoded_sub_seed = sub_seed.encode();

		if (Self::ID.len() + encoded_sub_seed.len()) > N {
			Err(error::Key::InvalidSubSeed)?;
		}

		Self::ID
			.iter()
			.cloned()
			.chain(encoded_sub_seed)
			.enumerate()
			.for_each(|(i, v)| result[i] = v);

		Ok(result)
	}
}

macro_rules! impl_keys {
	($(
		#[doc=$doc:expr]
		#[id=$id:expr]
		$name:ident($ty:ty),
	)+) => {
		$(
			#[doc=$doc]
			#[derive(Debug)]
			pub struct $name(pub $ty);
			impl Key for $name {
				type SubSeed = $ty;

				const ID: [u8; 4] = $id;

				fn sub_seed(self) -> Self::SubSeed {
					self.0
				}
			}
			impl TryFrom<&[u8]> for $name {
				type Error = Error;

				fn try_from(key: &[u8]) -> Result<Self> {
					let id_size = Self::ID.len();
					// Note:
					// These sub-seed types' mem size have the exact same size as `<$ty as Encode>::size_hint(&Default::default())`.
					// If introduce some complex types in the future, might need to use the formula above to calculate the size.
					let sub_seed_size = mem::size_of::<$ty>();

					if key.len() < id_size + sub_seed_size {
						Err(error::Key::InvalidKey)?;
					}

					let id = &key[..id_size];
					let sub_seed = &key[id_size..id_size + sub_seed_size];

					if id != Self::ID {
						Err(error::Key::InvalidKey)?;
					}

					Ok(Self(<$ty>::decode(&mut &*sub_seed).map_err(error::Generic::Codec)?))
				}
			}
		)+
	};
}
impl_keys! {
	#[doc="Unique identifier of a pallet, aka `ModuleId`."]
	#[id=*b"modl"]
	PalletId([u8; 8]),
	#[doc="Unique identifier of a parachain."]
	#[id=*b"para"]
	ParaId(u32),
	#[doc="Unique identifier of a sibling chain."]
	#[id=*b"sibl"]
	SiblId(u32),
}
impl Display for PalletId {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "PalletId({})", String::from_utf8_lossy(&self.0))
	}
}
impl Display for ParaId {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "ParaId({})", &self.0)
	}
}
impl Display for SiblId {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "SiblId({})", &self.0)
	}
}
