// hack-ink
use super::*;

#[test]
fn ss58_address_of_should_work() {
	assert_eq!(
		ss58_address_of(
			&array_bytes::hex2bytes_unchecked(
				"0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747"
			),
			"Polkadot",
		)
		.unwrap(),
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
		array_bytes::hex2bytes_unchecked(
			"0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747"
		)
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
