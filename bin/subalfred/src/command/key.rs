// std
use std::{borrow::Cow, fmt::Write};
// crates.io
use clap::{Args, ValueEnum};
// hack-ink
use crate::prelude::*;
use subalfred_core::{
	key::{Key, PalletId, ParaId, SiblId},
	ss58::{self, Address},
};

type ChainId = u32;

/// Calculate the public key/SS58 address of the SS58 address/public key.
#[derive(Debug, Args)]
pub(crate) struct KeyCmd {
	/// Public key/SS58 address.
	#[arg(required = true, value_name = "PUBLIC KEY/SS58 ADDRESS")]
	key: String,
	/// Key type.
	#[arg(value_enum, long, value_name = "KEY TYPE")]
	r#type: Option<KeyType>,
	/// Network name.
	#[arg(long, value_name = "NAME", default_value = "Substrate", conflicts_with = "list_all")]
	network: String,
	/// List all the networks' addresses.
	#[arg(long, num_args = 0, conflicts_with = "network")]
	list_all: bool,
	/// Show network(s) prefix(es).
	#[arg(long, num_args = 0)]
	show_prefix: bool,
}
impl KeyCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { key, r#type, network, list_all, show_prefix } = self;
		let key = if let Some(r#type) = r#type {
			Cow::Owned(array_bytes::bytes2hex("0x", &r#type.to_key::<32>(key)?))
		} else {
			Cow::Borrowed(key)
		};

		if *list_all {
			let (public_key, mut hex_public_key, addresses) = ss58::all(&key)?;
			let max_length = addresses.iter().map(|addr| addr.network.len()).max().unwrap_or(0);

			if let Ok(special_key) = try_get_key_type_from_public_key(public_key) {
				write!(hex_public_key, " {special_key}")?;
			}

			println!("public-key {hex_public_key}");

			if *show_prefix {
				addresses.into_iter().for_each(|Address { network, prefix, value }| {
					println!("{network:width$} {prefix:5} {value}", width = max_length)
				});
			} else {
				addresses.into_iter().for_each(|Address { network, value, .. }| {
					println!("{network:width$} {value}", width = max_length)
				});
			}
		} else {
			let (public_key, mut hex_public_key, Address { network, prefix, value }) =
				ss58::of(&key, network)?;

			if let Ok(special_key) = try_get_key_type_from_public_key(public_key) {
				write!(hex_public_key, " {special_key}")?;
			}

			println!("public-key {hex_public_key}",);

			if *show_prefix {
				println!("{network} {prefix} {value}");
			} else {
				println!("{network} {value}");
			}
		}

		Ok(())
	}
}

#[derive(Clone, Debug, ValueEnum)]
enum KeyType {
	Pallet,
	Parachain,
	Sibling,
}
impl KeyType {
	fn to_key<const N: usize>(&self, s: &str) -> Result<[u8; N]> {
		Ok(match self {
			KeyType::Pallet =>
				PalletId(array_bytes::slice2array(s.as_bytes()).map_err(quick_err)?).to_key()?,
			KeyType::Parachain => ParaId(s.parse::<ChainId>()?).to_key()?,
			KeyType::Sibling => SiblId(s.parse::<ChainId>()?).to_key()?,
		})
	}
}

fn try_get_key_type_from_public_key(public_key: impl AsRef<[u8]>) -> Result<String> {
	let public_key = public_key.as_ref();

	Ok(PalletId::try_from(public_key)
		.map(|k| ToString::to_string(&k))
		.or_else(|_| ParaId::try_from(public_key).map(|k| ToString::to_string(&k)))
		.or_else(|_| SiblId::try_from(public_key).map(|k| ToString::to_string(&k)))?)
}
