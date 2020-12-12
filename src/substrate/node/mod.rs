pub mod metadata;
pub mod version;

// --- crates.io ---
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct RpcResult {
	pub result: Value,
}
impl RpcResult {
	pub fn into_inner<T: DeserializeOwned>(self) -> T {
		serde_json::from_value(self.result).unwrap()
	}
}
