//! Substrate-like API collections.

// subalfred
use crate::prelude::*;

///  Substrate-like basic API collections.
#[async_trait::async_trait]
pub trait BasicApi {
	/// TODO: doc
	async fn get_block_hash<BlockNumber>(
		&self,
		block_number: Option<BlockNumber>,
	) -> Result<String>
	where
		BlockNumber: Send + serde::Serialize;

	/// TODO: doc
	async fn get_finalized_head(&self) -> Result<String>;

	/// TODO: doc
	async fn get_header<BlockNumber, Hash>(
		&self,
		hash: Option<Hash>,
	) -> Result<subruntimer::Header<BlockNumber, Hash>>
	where
		BlockNumber: Send + serde::de::DeserializeOwned,
		Hash: Send + serde::Serialize + serde::de::DeserializeOwned;

	/// TODO: doc
	async fn get_runtime_metadata(&self) -> Result<String>;

	/// TODO: doc
	async fn get_runtime_version<Hash>(
		&self,
		at: Option<Hash>,
	) -> Result<subversioner::RuntimeVersion>
	where
		Hash: Send + serde::Serialize;

	/// TODO: doc
	async fn get_pairs_paged(
		&self,
		prefix: substorager::StorageKey,
		at: Option<String>,
	) -> Result<Vec<(String, String)>>;

	// async fn get_keys_paged(&self, prefix: substorager::StorageKey, at: Option<String>) ->
	// Result<Vec<String>>;
}
