// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::ss58::{self, Address};

/// Convert the PUBLIC KEY/SS58 ADDRESS from SS58 ADDRESS/PUBLIC KEY.
#[derive(Debug, Args)]
pub struct AddressCmd {
	/// SS58 address or public key.
	#[clap(required = true, value_name = "PUBLIC KEY/SS58 ADDRESS")]
	address: String,
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
impl AddressCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { address, network, list_all, show_prefix } = self;

		if *list_all {
			let (public_key, addresses) = ss58::all(address)?;
			let max_length =
				addresses.iter().map(|address| address.network.len()).max().unwrap_or(0);

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
			let (public_key, Address { network, prefix, value }) = ss58::of(address, network)?;

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
