// hack-ink
use crate::*;
use subcryptor::Network;

impl Executor {
	pub fn account(account: &str, network: Option<&str>) {
		// TODO: simple output
		if let Some(network) = network {
			println!("{}", parse_account(account, network));
		} else {
			let accounts = parse_accounts(account);
			let max_width = accounts
				.iter()
				.map(|account| account.0.len())
				.max()
				.unwrap();

			for account in accounts {
				println!("{:>width$}: {}", account.0, account.1, width = max_width);
			}
		}
	}
}

#[allow(unused)]
pub fn parse_account(account: &str, network: &str) -> String {
	// TODO: parse with specific network
	todo!()
}

pub fn parse_accounts(account: &str) -> Vec<(String, String)> {
	let mut accounts = vec![];
	let public_key;

	if account.len() == 48 {
		public_key = subcryptor::into_public_key(account);

		accounts.push((
			"Public Key".into(),
			array_bytes::bytes2hex("0x", &public_key),
		));
	} else {
		public_key = array_bytes::hex2bytes(account).unwrap();

		accounts.push((
			"Public Key".into(),
			if account.starts_with("0x") {
				account.into()
			} else {
				format!("0x{}", account)
			},
		));
	}

	for &(network, prefix) in Network::PREFIXES {
		accounts.push((
			network.into(),
			subcryptor::into_ss58_address(&public_key, prefix),
		));
	}

	accounts
}
