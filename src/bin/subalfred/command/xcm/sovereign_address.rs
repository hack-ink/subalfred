// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred::core::xcm::sovereign_address;

/// Calculate the sovereign account addresses.
#[derive(Debug, Args)]
pub struct SovereignAddressCmd {
	/// Parachain ID.
	#[clap(value_name = "PARA_ID", required = true)]
	para_id: u32,
	/// Whether the chain's account type is `AccountId20`.
	#[clap(long, takes_value = false)]
	account_id_20: bool,
}
impl SovereignAddressCmd {
	pub fn run(&self) -> AnyResult<()> {
		let Self { para_id, account_id_20 } = self;
		let (relaychain_sovereign_address, parachain_sovereign_address) = if *account_id_20 {
			sovereign_address::of::<20>(*para_id)
		} else {
			sovereign_address::of::<32>(*para_id)
		};

		println!("Relaychain sovereign address {relaychain_sovereign_address}");
		println!("Parachain sovereign address  {parachain_sovereign_address}");

		Ok(())
	}
}
