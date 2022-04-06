// crates.io
use clap::Parser;
// hack-ink
use crate::*;

#[derive(Debug, Parser)]
pub struct AccountCmd {
	#[clap(
		required = true,
		takes_value = true,
		value_name = "PUBLIC KEY/SS58 ADDRESS"
	)]
	account: String,
	#[clap(short, long, takes_value = true, value_name = "NAME")]
	network: Option<String>,
}
impl Run for AccountCmd {
	fn run(&self) -> AnyResult<()> {
		let Self { account, network } = self;

		Executor::account(account, network.as_ref().map(AsRef::as_ref));

		Ok(())
	}
}
