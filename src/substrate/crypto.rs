// --- crates.io ---
use subcryptor::NETWORK;
// --- subalfred ---
use crate::Subalfred;

impl Subalfred {
	pub fn accounts(account: &str) -> Vec<(String, String)> {
		let mut accounts = vec![];
		let public_key;

		if account.len() == 48 {
			public_key = subcryptor::into_public_key(account);

			accounts.push(("Public Key".into(), array_bytes::hex_str("0x", &public_key)));
		} else {
			public_key = array_bytes::bytes(account).unwrap();

			accounts.push((
				"Public Key".into(),
				if account.starts_with("0x") {
					account.into()
				} else {
					format!("0x{}", account)
				},
			));
		}

		for network in &NETWORK {
			accounts.push((
				network.0.into(),
				subcryptor::into_ss58_address(&public_key, network.1),
			));
		}

		accounts
	}
}
