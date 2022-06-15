// std
use std::borrow::Cow;
// crates.io
use clap::{ArgEnum, Args};
// hack-ink
use crate::prelude::*;
use subalfred::core::{
	key::KeyTypeId,
	ss58::{self, Address},
};

// TODO: detect if it is a pallet account or sovereign account

/// Convert the public key/ss58 address from ss58 address/public key.
#[derive(Debug, Args)]
pub struct KeyCmd {
	/// Public key or SS58 address.
	#[clap(required = true, value_name = "PUBLIC KEY/SS58 ADDRESS")]
	key: String,
	/// The key type.
	#[clap(arg_enum, long, value_name = "KEY TYPE")]
	key_type: Option<KeyType>,
	/// Network address format.
	/// If not set, the default network is `Substrate`.
	#[clap(long, value_name = "NAME", default_value = "Substrate", conflicts_with = "list-all")]
	network: String,
	/// List all the networks' addresses.
	#[clap(long, takes_value = false, conflicts_with = "network")]
	list_all: bool,
	/// Show the network(s)' prefix(es).
	#[clap(long, takes_value = false)]
	show_prefix: bool,
}
impl KeyCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { key, key_type, network, list_all, show_prefix } = self;
		let key = if let Some(key_type) = key_type {
			Cow::Owned(array_bytes::bytes2hex(
				"0x",
				&match key_type {
					KeyType::Pallet => KeyTypeId::pallet().to_key::<32>(key.as_bytes())?,
					KeyType::Parachain => KeyTypeId::parachain().to_key::<32>(key.as_bytes())?,
					KeyType::Sibling => KeyTypeId::sibling().to_key::<32>(key.as_bytes())?,
				},
			))
		} else {
			Cow::Borrowed(key)
		};

		if *list_all {
			let (public_key, addresses) = ss58::all(&key)?;
			let max_length = addresses.iter().map(|addr| addr.network.len()).max().unwrap_or(0);

			println!("Public-key {public_key}");

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
			let (public_key, Address { network, prefix, value }) = ss58::of(&key, network)?;

			println!("Public-key {public_key}");

			if *show_prefix {
				println!("{network} {prefix} {value}");
			} else {
				println!("{network} {value}");
			}
		}

		Ok(())
	}
}

#[derive(Clone, Debug, ArgEnum)]
pub enum KeyType {
	Pallet,
	Parachain,
	Sibling,
}
