//! Subalfred's core key library.

mod keystore;
pub use keystore::*;

// std
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	mem,
};
// crates.io
use parity_scale_codec::{Decode, Encode};
// subalfred
use crate::prelude::*;

/// Substrate-like key trait.
pub trait Key
where
	Self: Sized,
{
	/// Key Id.
	const ID: [u8; 4];

	/// Seed type.
	type Seed: Encode;

	/// Seed, used to derive the key.
	fn seed(self) -> Self::Seed;

	/// Derive the key from the given seed.
	fn to_key<const N: usize>(self) -> Result<[u8; N]> {
		self.to_key_with_sub_seed(())
	}

	/// Derive the key from the given seed and sub-seed.
	fn to_key_with_sub_seed<S, const N: usize>(self, sub_seed: S) -> Result<[u8; N]>
	where
		S: Encode,
	{
		let mut result = [0; N];
		let seed = (self.seed(), sub_seed).encode();

		if (Self::ID.len() + seed.len()) > N {
			Err(error::Key::InvalidSubSeed)?;
		}

		Self::ID.iter().cloned().chain(seed).enumerate().for_each(|(i, v)| result[i] = v);

		Ok(result)
	}

	// TODO: add `from_key` method.
	// TODO: add `from_key_with_sub_seed` method.
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
				type Seed = $ty;

				const ID: [u8; 4] = $id;

				fn seed(self) -> Self::Seed {
					self.0
				}
			}
			impl TryFrom<&[u8]> for $name {
				type Error = Error;

				fn try_from(key: &[u8]) -> Result<Self> {
					let id_size = Self::ID.len();
					// Please note that the memory size of these sub-seed types is exactly the same as
					// `<$ty as Encode>::size_hint(&Default::default())`.
					// If complex types are introduced in the future,
					// it may be necessary to use the aforementioned formula to calculate their size.
					let seed_size = mem::size_of::<$ty>();

					if key.len() < id_size + seed_size {
						Err(error::Key::InvalidKey)?;
					}

					let id = &key[..id_size];
					let seed = &key[id_size..id_size + seed_size];

					if id != Self::ID {
						Err(error::Key::InvalidKey)?;
					}

					Ok(Self(<$ty>::decode(&mut &*seed).map_err(error::Generic::Codec)?))
				}
			}
		)+
	};
}
impl_keys! {
	#[doc="Identifier of a pallet, aka `ModuleId`."]
	#[id=*b"modl"]
	PalletId([u8; 8]),
	#[doc="Identifier of a parachain."]
	#[id=*b"para"]
	ParaId(u32),
	#[doc="Identifier of a sibling chain."]
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
#[test]
fn to_key_with_sub_seed_should_work() {
	// This is the crowdloan address and fund index for parachain 2025.
	// Data retrieved from Polkadot#18549460.
	assert_eq!(
		array_bytes::bytes2hex(
			"0x",
			PalletId(*b"py/cfund").to_key_with_sub_seed::<_, 32>(75_u32).unwrap()
		),
		"0x6d6f646c70792f6366756e644b00000000000000000000000000000000000000"
	);
}
