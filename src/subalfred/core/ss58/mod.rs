#[cfg(test)] mod test;

// hack-ink
use crate::core::{error, Result};
use subcryptor::{ss58_registry::ALL_SS58_ADDRESS_FORMAT_NAMES, Sr25519};

/// Generate the public key and the specific network address for address.
///
/// `address` could be public key or SS58 address.
/// `network` is case insensitive.
pub fn of(address: &str, network: &str) -> Result<(String, String)> {
	let public_key = recover_public_key(address)?;
	let hex_public_key = array_bytes::bytes2hex("0x", &public_key);
	let address = subcryptor::ss58_address_of(&public_key, network)
		.map_err(error::Ss58::CalculateSs58AddressFailed)?;

	Ok((hex_public_key, address))
}

/// Generate the public key and all the network addresses for the address.
///
/// `address` could be public key or SS58 address.
pub fn all(address: &str) -> Result<(String, Vec<(&'static str, String)>)> {
	let public_key = recover_public_key(address)?;
	let hex_public_key = array_bytes::bytes2hex("0x", &public_key);
	let mut addresses = Vec::new();

	for network in ALL_SS58_ADDRESS_FORMAT_NAMES {
		addresses.push((
			network,
			subcryptor::ss58_address_of(&public_key, network)
				.map_err(error::Ss58::CalculateSs58AddressFailed)?,
		));
	}

	Ok((hex_public_key, addresses))
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
