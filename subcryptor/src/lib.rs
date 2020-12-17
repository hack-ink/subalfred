#[cfg(feature = "full-crypto")]
pub mod full_crypto {
	// --- crates.io ---
	#[cfg(feature = "codec")]
	use parity_scale_codec::Encode;

	pub type PublicKey = [u8; 32];
	pub type Signature = [u8; 64];

	pub const SIGNING_CTX: &[u8] = b"substrate";

	#[cfg_attr(feature = "codec", derive(Encode))]
	pub enum MultiSignature {
		_Ed25519,
		Sr25519(Signature),
		_Ecdsa,
	}
	impl From<Signature> for MultiSignature {
		fn from(signature: Signature) -> Self {
			Self::Sr25519(signature)
		}
	}
}

// --- crates.io ---
#[cfg(feature = "full-crypto")]
pub use full_crypto::*;
#[cfg(feature = "full-crypto")]
pub use schnorrkel;

// --- crates.io ---
use base58::{FromBase58, ToBase58};
use blake2_rfc::blake2b::Blake2b;

pub const NETWORK: [(&str, u8); 32] = [
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
