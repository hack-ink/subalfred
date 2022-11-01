// hack-ink
use super::*;

const PUBLIC_KEY: &[u8] = &[
	180, 247, 240, 59, 235, 197, 110, 190, 150, 188, 82, 234, 94, 211, 21, 157, 69, 160, 206, 58,
	141, 127, 8, 41, 131, 195, 62, 241, 51, 39, 71, 71,
];

#[test]
fn ss58_address_of_should_work() {
	assert_eq!(
		ss58_address_of(PUBLIC_KEY, "Polkadot",).unwrap(),
		(0, "156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y".into())
	);
}
#[test]
fn ss58_address_of_should_fail() {
	assert_eq!(
		ss58_address_of(&[], "invalid network",).unwrap_err().to_string(),
		"[subcryptor] unsupported network, \"invalid network\""
	);
}

#[test]
fn public_key_of_should_work() {
	assert_eq!(
		public_key_of::<Sr25519>("156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y").unwrap(),
		PUBLIC_KEY
	);
}

#[test]
fn public_key_of_should_fail() {
	assert_eq!(
		public_key_of::<Sr25519>("").unwrap_err().to_string(),
		"[subcryptor] invalid ss58 address, \"\""
	);
	assert_eq!(
		public_key_of::<Sr25519>("?56HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y")
			.unwrap_err()
			.to_string(),
		"[subcryptor] from base58 error, InvalidBase58Character('?', 0)"
	);
	assert_eq!(
		public_key_of::<Sr25519>("56HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y")
			.unwrap_err()
			.to_string(),
		"[subcryptor] invalid prefix, 180"
	);
	assert_eq!(
		public_key_of::<Sr25519>("15").unwrap_err().to_string(),
		"[subcryptor] invalid ss58 address, \"15\""
	);
}
