//! Minimal implementation of Substrate version.

#![deny(missing_docs)]

// crates.io
#[cfg(feature = "serde")] use serde::Deserialize;

/// Runtime version.
///
/// Substrate reference(s):
/// - https://github.com/paritytech/substrate/blob/c4d36065764ee23aeb3ccd181c4b6ecea8d2447a/primitives/version/src/lib.rs#L152-L215
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(rename_all = "camelCase"))]
pub struct RuntimeVersion {
	#[allow(missing_docs)]
	pub spec_name: String,
	#[allow(missing_docs)]
	pub impl_name: String,
	#[allow(missing_docs)]
	pub authoring_version: u32,
	#[allow(missing_docs)]
	pub spec_version: u32,
	#[allow(missing_docs)]
	pub impl_version: u32,
	#[allow(missing_docs)]
	pub transaction_version: u32,
	#[allow(missing_docs)]
	pub state_version: u8,
}
