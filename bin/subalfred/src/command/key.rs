// std
use std::borrow::Cow;
// crates.io
use clap::{Args, ValueEnum};
// hack-ink
use crate::{command::shared::JsonOutput, prelude::*};
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
	#[arg(value_enum, long, value_name = "TYPE")]
	r#type: Option<KeyType>,
	/// Network name.
	#[arg(long, value_name = "NAME", default_value = "Substrate", conflicts_with = "list_all")]
	network: String,
	/// List all the networks' addresses.
	#[arg(long, conflicts_with = "network")]
	list_all: bool,
	/// Show network(s) prefix(es).
	#[arg(long)]
	show_prefix: bool,
	#[command(flatten)]
	json_output: JsonOutput,
}
impl KeyCmd {
	fn run_(&self) -> Result<String> {
		let Self {
			key,
			r#type,
			network,
			list_all,
			show_prefix,
			json_output: JsonOutput { json_output },
		} = self;
		let key = if let Some(r#type) = r#type {
			Cow::Owned(array_bytes::bytes2hex("0x", &r#type.to_key::<32>(key)?))
		} else {
			Cow::Borrowed(key)
		};

		Ok(if *list_all {
			let (public_key, hex_public_key, addresses) = ss58::all(&key)?;
			let sub_seed = sub_seed_from_public_key(public_key).unwrap_or_default();

			if *json_output {
				build_json_output(&sub_seed, &hex_public_key, *show_prefix, &addresses)
			} else {
				build_plain_output(&sub_seed, &hex_public_key, *show_prefix, &addresses)
			}
		} else {
			let (public_key, hex_public_key, address) = ss58::of(&key, network)?;
			let sub_seed = sub_seed_from_public_key(public_key).unwrap_or_default();

			if *json_output {
				build_json_output(&sub_seed, &hex_public_key, *show_prefix, &[address])
			} else {
				build_plain_output(&sub_seed, &hex_public_key, *show_prefix, &[address])
			}
		})
	}

	pub(crate) fn run(&self) -> Result<()> {
		let result = self.run_()?;

		println!("{result}");

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

// TODO: change result to `Result<Option<String>>`
// TODO: if the key is not a specific key then return `Ok(None)`
fn sub_seed_from_public_key(public_key: impl AsRef<[u8]>) -> Result<String> {
	let public_key = public_key.as_ref();

	Ok(PalletId::try_from(public_key)
		.map(|k| ToString::to_string(&k))
		.or_else(|_| ParaId::try_from(public_key).map(|k| ToString::to_string(&k)))
		.or_else(|_| SiblId::try_from(public_key).map(|k| ToString::to_string(&k)))?)
}

fn build_plain_output(
	sub_seed: &str,
	public_key: &str,
	show_prefix: bool,
	addresses: &[Address],
) -> String {
	format!(
		"\
		sub-seed {sub_seed}\n\
		public-key {public_key}\n\
		{}\
		",
		if show_prefix {
			addresses
				.iter()
				.map(|Address { network, prefix, value }| format!("{network} {prefix:5} {value}"))
				.collect::<Vec<_>>()
				.join("\n")
		} else {
			addresses
				.iter()
				.map(|Address { network, value, .. }| format!("{network} {value}"))
				.collect::<Vec<_>>()
				.join("\n")
		}
	)
}

fn build_json_output(
	sub_seed: &str,
	public_key: &str,
	show_prefix: bool,
	addresses: &[Address],
) -> String {
	serde_json::json!({
		"sub-seed": sub_seed,
		"public-key": public_key,
		"addresses":  if show_prefix {
			addresses.iter().map(|Address { network, prefix, value }| {
				serde_json::json!({
					"network": network,
					"prefix": prefix,
					"address": value
				})
			}).collect::<Vec<_>>()
		} else {
			addresses.iter().map(|Address { network, value, .. }| {
				serde_json::json!({
					"network": network,
					"address": value
				})
			}).collect::<Vec<_>>()
		}
	})
	.to_string()
}

#[test]
fn key_cmd_should_work() {
	let cmd = KeyCmd {
		key: "py/trsry".into(),
		r#type: Some(KeyType::Pallet),
		network: "Polkadot".into(),
		list_all: false,
		show_prefix: true,
		json_output: JsonOutput { json_output: true },
	};

	assert_eq!(
		cmd.run_().unwrap(),
		"{\
			\"addresses\":[\
				{\
					\"address\":\"13UVJyLnbVp9RBZYFwFGyDvVd1y27Tt8tkntv6Q7JVPhFsTB\",\
					\"network\":\"Polkadot\",\
					\"prefix\":0\
				}\
			],\
			\"public-key\":\
			\"0x6d6f646c70792f74727372790000000000000000000000000000000000000000\",\
			\"sub-seed\":\"PalletId(py/trsry)\
		\"}"
	);
}
