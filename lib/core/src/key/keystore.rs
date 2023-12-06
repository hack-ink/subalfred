// crates.io
use serde::Deserialize;
// subalfred
use crate::prelude::*;
use subcryptor::constant::SECRET_KEY_LEN;

/// JSON keystore structure.
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct KeystoreJson {
	pub encoded: String,
	pub encoding: Encoding,
}
impl KeystoreJson {
	/// Decrypt the keystore.
	pub fn decrypt(&self, passphrase: &str) -> Result<[u8; SECRET_KEY_LEN]> {
		Ok(subcryptor::decrypt_keystore(
			passphrase.as_bytes(),
			&subcryptor::base64_decode(&self.encoded)?,
			&self.encoding.r#type,
		)?)
	}
}
#[test]
fn decrypt_should_work() {
	let alice_keystore = r#"{"encoded":"paO3SvbXf/ktnJwuxB5Jn3L5Yxrv9JPhOLA4dplOFu8AgAAAAQAAAAgAAACuIZoAe4sfEiV55sUsZaGq9cCwddaqYdBE7rMSfZfYd7kjgw5YUIo7QB6e+PVY+q6Du8uRSxOOjR5++LTEavlppjbkEce38/MrsDmrKccUqxu+hjv8Rt/RK77btcSuWDm6CA5bgX3y2lKeYqJEidMbP68dSKEN8uOoEdOuCdAFCH1BHCKPhOcu80RNcQ9puZDqZdLoPxc7QDlF9LS5","encoding":{"content":["pkcs8","sr25519"],"type":["scrypt","xsalsa20-poly1305"],"version":"3"},"address":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","meta":{"genesisHash":"","name":"Alice","whenCreated":1701855108366}}"#;
	let alice_secret_key = "0x98319d4ff8a9508c4bb0cf0b5a78d760a0b2082c02775e6e82370816fedfff48925a225d97aa00682d6a59b95b18780c10d7032336e88f3442b42361f4a66011";

	assert_eq!(
		array_bytes::bytes2hex(
			"0x",
			serde_json::from_str::<KeystoreJson>(alice_keystore)
				.unwrap()
				.decrypt("456123")
				.unwrap()
		),
		alice_secret_key
	);
}
/// JSON keystore encoding structure.
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct Encoding {
	pub r#type: Vec<String>,
}
