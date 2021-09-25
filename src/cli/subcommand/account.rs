// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::{cli::Run, AnyResult, Subalfred};

#[derive(Debug, StructOpt)]
pub struct AccountCmd {
	#[structopt(
		required = true,
		takes_value = true,
		value_name = "PUBLIC KEY/SS58 ADDRESS"
	)]
	account: String,
	#[structopt(short, long, takes_value = true, value_name = "NAME")]
	network: Option<String>,
}
impl Run for AccountCmd {
	fn run(&self) -> AnyResult<()> {
		let Self { account, network } = self;
		if let Some(network) = network {
			println!("{}", Subalfred::account(account));
		} else {
			let accounts = Subalfred::accounts(account);
			let max_width = accounts
				.iter()
				.map(|account| account.0.len())
				.max()
				.unwrap();

			for account in accounts {
				println!("{:>width$}: {}", account.0, account.1, width = max_width);
			}
		}

		Ok(())
	}
}
