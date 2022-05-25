#[cfg(test)] mod test;

// hack-ink
use crate::core::{error, Result};
use subcryptor::Network;

// TODO: `AccountId20`
/// Generate the public key and the specific network address for address.
///
/// `address` could be public key or SS58 address.
/// `network` is case insensitive.
pub fn of(address: &str, network: &str) -> Result<(String, String)> {
	let public_key = recover_public_key(address)?;
	let network_lc = network.to_ascii_lowercase();
	let prefix = Network::PREFIXES
		.iter()
		.find(|(n, _)| n.to_ascii_lowercase() == network_lc)
		.ok_or_else(|| error::Ss58::UnsupportedNetwork(network.into()))?
		.1;
	let hex_public_key = array_bytes::bytes2hex("0x", &public_key);
	let address = subcryptor::into_ss58_address(&public_key, prefix);

	Ok((hex_public_key, address))
}

/// Generate the public key and all the network addresses for the address.
///
/// `address` could be public key or SS58 address.
pub fn all(address: &str) -> Result<(String, Vec<(String, String)>)> {
	let public_key = recover_public_key(address)?;
	let hex_public_key = array_bytes::bytes2hex("0x", &public_key);
	let mut addresses = Vec::new();

	Network::PREFIXES.iter().for_each(|&(network, prefix)| {
		addresses.push((network.into(), subcryptor::into_ss58_address(&public_key, prefix)));
	});

	Ok((hex_public_key, addresses))
}

/// Recover the public key from the given address.
/// NO-OP, If the address is already a public key.
///
/// `address` could be public key or SS58 address.
fn recover_public_key(address: &str) -> Result<Vec<u8>> {
	match address.len() {
		48 => Ok(subcryptor::into_public_key(address)),
		64 if !address.starts_with("0x") => Ok(array_bytes::hex2bytes(address)
			.map_err(|_| error::Ss58::InvalidAddress(address.into()))?),
		66 if address.starts_with("0x") => Ok(array_bytes::hex2bytes(address)
			.map_err(|_| error::Ss58::InvalidAddress(address.into()))?),
		_ => Err(error::Ss58::InvalidAddress(address.into()))?,
	}
}
