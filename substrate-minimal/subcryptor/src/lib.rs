//! Minimal implementation of Substrate crypto.

#![deny(missing_docs)]

#[cfg(test)] mod test;

mod error;
pub use error::Error;

pub use ss58_registry;

// crates.io
use base58::{FromBase58, ToBase58};
use blake2_rfc::blake2b::Blake2b;
use ss58_registry::Ss58AddressFormat;

/// Main result.
pub type Result<T> = std::result::Result<T, Error>;

/// Key trait.
///
/// The only usage of this trait is to indicate the key's bytes length.
pub trait Key {
	/// Key's bytes length.
	const LEN: usize;
}

/// ECDSA key type.
pub struct Ecdsa;
impl Key for Ecdsa {
	const LEN: usize = 33;
}
/// ED25519 key type.
pub struct Ed25519;
impl Key for Ed25519 {
	const LEN: usize = 32;
}
/// SR25519 key type.
pub struct Sr25519;
impl Key for Sr25519 {
	const LEN: usize = 32;
}

/// Get the specific network SS58 address of the public key.
///
/// Substrate reference(s):
/// - <https://github.com/paritytech/substrate/blob/c4d36065764ee23aeb3ccd181c4b6ecea8d2447a/primitives/core/src/crypto.rs#L315-L336>
pub fn ss58_address_of(public_key: &[u8], network: &str) -> Result<(u16, String)> {
	let network = Ss58AddressFormat::try_from(network)
		.map_err(|_| Error::UnsupportedNetwork(network.into()))?;
	let prefix = u16::from(network);
	let mut bytes = match prefix {
		0..=63 => vec![prefix as u8],
		64..=16_383 => {
			let first = ((prefix & 0b0000_0000_1111_1100) as u8) >> 2;
			let second = ((prefix >> 8) as u8) | ((prefix & 0b0000_0000_0000_0011) as u8) << 6;

			vec![first | 0b01000000, second]
		},
		_ => Err(Error::UnsupportedNetwork(network.into()))?,
	};

	bytes.extend(public_key);

	let blake2b = {
		let mut context = Blake2b::new(64);
		context.update(b"SS58PRE");
		context.update(&bytes);

		context.finalize()
	};

	bytes.extend(&blake2b.as_bytes()[0..2]);

	Ok((prefix, bytes.to_base58()))
}

/// Get the public key of the SS58 address.
///
/// Substrate reference(s):
/// - <https://github.com/paritytech/substrate/blob/c4d36065764ee23aeb3ccd181c4b6ecea8d2447a/primitives/core/src/crypto.rs#L260-L302>
pub fn public_key_of<K>(ss58_address: &str) -> Result<Vec<u8>>
where
	K: Key,
{
	if ss58_address.is_empty() {
		return Err(Error::InvalidSs58Address(ss58_address.into()));
	}

	let bytes = ss58_address.from_base58().map_err(Error::FromBase58)?;
	let prefix_len = match bytes[0] {
		0..=63 => 1,
		64..=127 => 2,
		prefix => Err(Error::InvalidPrefix(prefix))?,
	};

	if bytes.len() < (K::LEN + prefix_len - 1) {
		Err(Error::InvalidSs58Address(ss58_address.into()))?;
	}

	Ok(bytes[prefix_len..K::LEN + prefix_len].to_vec())
}
