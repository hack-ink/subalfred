// --- crates.io ---
use base58::{FromBase58, ToBase58};
use blake2_rfc::blake2b::Blake2b;

const NETWORK: [(&'static str, u8); 32] = [
	("Polkadot", 0),
	("Kusama", 2),
	("KatalChain", 3),
	("Plasm", 5),
	("Bifrost", 6),
	("Edgeware", 7),
	("Karura", 8),
	("Reynolds", 9),
	("Acala", 10),
	("Laminar", 11),
	("Polymath", 12),
	("SubstraTee", 13),
	("Totem", 14),
	("Synesthesia", 15),
	("Kulupu", 16),
	("Dark", 17),
	("Darwinia", 18),
	("Geek", 19),
	("Stafi", 20),
	("DockTest", 21),
	("DockMain", 22),
	("ShiftNrg", 23),
	("Zero", 24),
	("Alphaville", 25),
	("Subsocial", 28),
	("Phala", 30),
	("Robonomics", 32),
	("DataHighway", 33),
	("Centrifuge", 36),
	("Nodle", 37),
	("Substrate", 42),
	("ChainX", 44),
];

pub fn parse_account(account: &str) -> Vec<(String, String)> {
	let mut accounts = vec![];
	let public_key;

	if account.len() == 48 {
		public_key = into_public_key(account);

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
		accounts.push((network.0.into(), into_ss58_address(&public_key, network.1)));
	}

	accounts
}

pub fn into_ss58_address(public_key: impl AsRef<[u8]>, network: u8) -> String {
	let mut bytes = {
		let mut data = vec![network];
		data.extend(public_key.as_ref());

		data
	};

	let blake2b = {
		let mut context = Blake2b::new(64);
		context.update(b"SS58PRE");
		context.update(&bytes);

		context.finalize()
	};
	bytes.extend(&blake2b.as_bytes()[0..2]);

	bytes.to_base58()
}

pub fn into_public_key(ss58_address: impl AsRef<str>) -> Vec<u8> {
	let bytes = ss58_address.as_ref().from_base58().unwrap();

	bytes[1..bytes.len() - 2].to_vec()
}
