//! Substrate keys implementations.

// crates.io
use parity_scale_codec::Encode;
// hack-ink
use crate::core::{error, Result};

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
