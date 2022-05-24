// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::ss58;

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
}
impl AddressCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { address, network, list_all } = self;

		if *list_all {
			let (public_key, addresses) = ss58::all(address)?;

			println!("Public key: {public_key}");

			// TODO: beautify output
			addresses.into_iter().for_each(|(network, address)| println!("{network}: {address}"));
		} else {
			let (public_key, address) = ss58::of(address, network)?;

			println!("Public key: {public_key}");
			println!("{network}: {address}");
		}

		Ok(())
	}
}
