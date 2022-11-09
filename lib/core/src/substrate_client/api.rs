//! Substrate-like API collections.
//!
//! Substrate reference(s):
//! - [RPC APIs](https://github.com/paritytech/substrate/tree/be259234bfee056bef970ac372e04a74411c5224/client/rpc-api)

// subalfred
use crate::prelude::*;

/// Substrate-like basic API collections.
#[async_trait::async_trait]
pub trait Apis {
	/// Check module's Substrate reference(s) for the detail.
	async fn get_block_hash<BlockNumber>(
		&self,
		block_number: Option<BlockNumber>,
	) -> Result<String>
	where
		BlockNumber: Send + serde::Serialize;

	/// Check module's Substrate reference(s) for the detail.
	async fn get_finalized_head(&self) -> Result<String>;

	/// Check module's Substrate reference(s) for the detail.
	async fn get_header<BlockNumber, Hash>(
		&self,
		hash: Option<Hash>,
	) -> Result<subruntimer::Header<BlockNumber, Hash>>
	where
		BlockNumber: Send + serde::de::DeserializeOwned,
		Hash: Send + serde::Serialize + serde::de::DeserializeOwned;

	/// Check module's Substrate reference(s) for the detail.
	async fn get_metadata<Hash>(&self, at: Option<Hash>) -> Result<String>
	where
		Hash: Send + serde::Serialize;

	/// Check module's Substrate reference(s) for the detail.
	async fn get_runtime_version<Hash>(
		&self,
		at: Option<Hash>,
	) -> Result<subversioner::RuntimeVersion>
	where
		Hash: Send + serde::Serialize;

	/// Check module's Substrate reference(s) for the detail.
	async fn get_pairs_paged(
		&self,
		prefix: substorager::StorageKey,
		at: Option<String>,
	) -> Result<Vec<(String, String)>>;
}
