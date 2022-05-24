// crates.io
use parity_scale_codec::Encode;

// Get sovereign addresses of the `para_id`.
pub fn of<const N: usize>(para_id: u32) -> (String, String) {
	let relaychain_sovereign_address = array_bytes::bytes2hex("0x", address::<N>(b"para", para_id));
	let parachain_sovereign_address = array_bytes::bytes2hex("0x", address::<N>(b"sibl", para_id));

	(relaychain_sovereign_address, parachain_sovereign_address)
}

/// Calculate the address from the given `key_type` and `para_id`.
fn address<const N: usize>(key_type: &[u8; 4], para_id: u32) -> [u8; N] {
	let mut address = [0; N];

	key_type.iter().cloned().chain(para_id.encode()).enumerate().for_each(|(i, v)| address[i] = v);

	address
}
