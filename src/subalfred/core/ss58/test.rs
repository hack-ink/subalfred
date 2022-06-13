// hack-ink
use super::*;

fn test_data() -> (Vec<u8>, String, Address<'static>) {
	let hex_public_key = "0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747";
	let public_key = array_bytes::hex2bytes_unchecked(hex_public_key);

	(
		public_key,
		hex_public_key.into(),
		Address {
			network: "Darwinia",
			prefix: 18,
			value: "2sG9veu8a88hH683YwKw8yQRWvoZxjgJmmYQHsp8sqDfxHHW".into(),
		},
	)
}

#[test]
fn of_should_work() {
	let (_, hex_public_key, address) = test_data();
	let expect_result = Ok((hex_public_key.clone(), address.clone()));

	assert_eq!(of(&hex_public_key, &address.network).map_err(|_| ()), expect_result);
	assert_eq!(of(&address.value, &address.network).map_err(|_| ()), expect_result);
}
#[test]
fn of_should_fail() {
	let (_, hex_public_key, address) = test_data();
	let network = "UnsupportedNetwork";
	let expect_result = Err(format!("[core::ss58] failed to calculate SS58 address"));

	assert_eq!(of(&hex_public_key, network).map_err(|e| e.to_string()), expect_result);
	assert_eq!(of(&address.value, network).map_err(|e| e.to_string()), expect_result);
}

// TODO: add testcase
#[test]
fn all_should_work() {}

#[test]
fn recover_public_key_should_work() {
	let (public_key, hex_public_key, address) = test_data();
	let expect_result = Ok(public_key);

	assert_eq!(recover_public_key(&hex_public_key).map_err(|_| ()), expect_result);
	assert_eq!(recover_public_key(&address.value).map_err(|_| ()), expect_result);
}
#[test]
fn recover_public_key_should_fail() {
	let address = "";
	let expect_result = Err(format!("[core::ss58] invalid address, {:?}", address));

	assert_eq!(recover_public_key(&address).map_err(|e| e.to_string()), expect_result);
}
