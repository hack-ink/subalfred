//! Substrate-Like API collections.

// subalfred
use crate::core::Result;
use substorager::StorageKey;

/// TODO: doc
#[async_trait::async_trait]
pub trait Api {
	/// TODO: doc
	async fn get_finalized_head(&self) -> Result<String>;

	/// TODO: doc
	async fn get_runtime_metadata(&self) -> Result<String>;

	/// TODO: doc
	async fn get_pairs_paged(
		&self,
		prefix: StorageKey,
		at: Option<String>,
	) -> Result<Vec<(String, String)>>;

	// async fn get_keys_paged(&self, prefix: StorageKey, at: Option<String>) ->
	// Result<Vec<String>>;
}
