// crates.io
#[cfg(feature = "serde")] use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(rename_all = "camelCase"))]
pub struct RuntimeVersion {
	pub spec_name: String,
	pub impl_name: String,
	pub authoring_version: u32,
	pub spec_version: u32,
	pub impl_version: u32,
	pub transaction_version: u32,
}
