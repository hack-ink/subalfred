//! SS58-prefix address algorithm implementation.

#[cfg(test)] mod test;

// hack-ink
use crate::core::prelude::*;
use subcryptor::{ss58_registry::Ss58AddressFormat, Sr25519};

/// Network address.
#[cfg_attr(test, derive(Clone, PartialEq, Eq))]
#[derive(Debug)]
pub struct Address<'a> {
	/// Network name.
	pub network: &'a str,
	/// Network prefix.
	pub prefix: u16,
	/// Address value.
	pub value: String,
}

/// Generate the public key and the specific network address for address.
///
/// `address` could be public key or SS58 address.
/// `network` is case insensitive.
pub fn of<'a>(address: &str, network: &'a str) -> Result<(Vec<u8>, String, Address<'a>)> {
	let public_key = recover_public_key(address)?;
	let hex_public_key = array_bytes::bytes2hex("0x", &public_key);
	let (prefix, address) = subcryptor::ss58_address_of(&public_key, network)
		.map_err(error::Ss58::CalculateSs58AddressFailed)?;

	Ok((public_key, hex_public_key, Address { network, prefix, value: address }))
}

/// Generate the public key and all the network addresses for the address.
///
/// `address` could be public key or SS58 address.
pub fn all(address: &str) -> Result<(Vec<u8>, String, Vec<Address>)> {
	let public_key = recover_public_key(address)?;
	let hex_public_key = array_bytes::bytes2hex("0x", &public_key);
	let mut addresses = Vec::new();

	for network in Ss58AddressFormat::all_names() {
		let (prefix, address) = subcryptor::ss58_address_of(&public_key, network)
			.map_err(error::Ss58::CalculateSs58AddressFailed)?;

		addresses.push(Address { network, prefix, value: address });
	}

	Ok((public_key, hex_public_key, addresses))
}

/// Recover the public key from the given address.
///
/// `address` could be public key or SS58 address.
/// NO-OP, If the `address` is already a public key.
fn recover_public_key(address: &str) -> Result<Vec<u8>> {
	match address.len() {
		// TODO: support more key types
		48 | 49 => Ok(subcryptor::public_key_of::<Sr25519>(address).map_err(|e| {
			error::Ss58::InvalidAddress {
				address: address.into(),
				source: Some(error::Ss58InvalidAddressSource::Subcryptor(e)),
			}
		})?),
		len => {
			if (len == 64 && !address.starts_with("0x")) || (len == 66 && address.starts_with("0x"))
			{
				Ok(array_bytes::hex2bytes(address).map_err(|e| error::Ss58::InvalidAddress {
					address: address.into(),
					source: Some(error::Ss58InvalidAddressSource::ArrayBytes(e)),
				})?)
			} else {
				Err(error::Ss58::InvalidAddress { address: address.into(), source: None })?
			}
		},
	}
}
