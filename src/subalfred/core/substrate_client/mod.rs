//! Substrate-Base API client implementation.

// TODO: maybe HTTP

mod api;
pub use api::BasicApi;

// std
use std::sync::Arc;
// crates.io
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc::{self, Sender};
// subalfred
use crate::core::{
	jsonrpc::ws::{Initializer, Ws},
	prelude::*,
};
use subrpcer::{chain, state};
use subruntimer::Header;
use substorager::StorageKey;
use subversioner::RuntimeVersion;

const PAGE_SIZE: usize = 512;

/// Substrate-Base API websocket client.
#[derive(Clone)]
pub struct Client {
	/// Websocket connection.
	ws: Arc<Ws>,
}
impl Client {
	/// Initialize the client with the given initializer.
	pub async fn initialize(initializer: Initializer, uri: &str) -> Result<Self> {
		Ok(Self { ws: Arc::new(initializer.connect(uri).await?) })
	}
}
impl Client {
	async fn get_keys_paged_concurrent(
		&self,
		prefix: StorageKey,
		at: Option<String>,
		tx: Sender<Vec<String>>,
	) -> Result<()> {
		let prefix = prefix.to_string();
		let mut start_key = None::<String>;
		let mut keys_count = 0;

		// Debug.
		// let mut i = 0;

		loop {
			let response = self
				.ws
				.request::<Vec<String>, _>(state::get_keys_paged_raw(
					&prefix,
					PAGE_SIZE,
					start_key.as_ref(),
					at.as_ref(),
				))
				.await?;
			let downloaded_keys = response.result;
			let downloaded_keys_count = downloaded_keys.len();

			keys_count += downloaded_keys_count;

			if let Some(key) = downloaded_keys.last() {
				start_key = Some(key.to_owned());
			} else {
				tracing::warn!(
					"no keys found under prefix({prefix}) start_key({})",
					start_key.unwrap_or_default()
				);

				return Ok(());
			}

			tracing::trace!("fetched {keys_count} keys");

			tx.send(downloaded_keys).await.map_err(|_| error::Tokio::MpscSend)?;

			if downloaded_keys_count < PAGE_SIZE {
				return Ok(());
			}

			// Debug.
			// if i < 5 {
			// 	i += 1;
			// } else {
			// 	return Ok(());
			// }
		}
	}
}
#[async_trait::async_trait]
impl BasicApi for Client {
	async fn get_block_hash<BlockNumber>(&self, block_number: Option<BlockNumber>) -> Result<String>
	where
		BlockNumber: Send + Serialize,
	{
		Ok(self.ws.request(chain::get_block_hash_raw(block_number)).await?.result)
	}

	async fn get_finalized_head(&self) -> Result<String> {
		Ok(self.ws.request(chain::get_finalized_head_raw()).await?.result)
	}

	async fn get_runtime_metadata(&self) -> Result<String> {
		Ok(self.ws.request(state::get_metadata_raw()).await?.result)
	}

	async fn get_header<BlockNumber, Hash>(
		&self,
		hash: Option<Hash>,
	) -> Result<Header<BlockNumber, Hash>>
	where
		BlockNumber: Send + DeserializeOwned,
		Hash: Send + Serialize + DeserializeOwned,
	{
		Ok(self.ws.request(chain::get_header_raw(hash)).await?.result)
	}

	async fn get_runtime_version<Hash>(&self, at: Option<Hash>) -> Result<RuntimeVersion>
	where
		Hash: Send + Serialize,
	{
		Ok(self.ws.request(state::get_runtime_version_raw(at)).await?.result)
	}

	async fn get_pairs_paged(
		&self,
		prefix: substorager::StorageKey,
		at: Option<String>,
	) -> Result<Vec<(String, String)>> {
		let (get_keys_paged_tx, mut get_keys_paged_rx) = mpsc::channel(PAGE_SIZE);

		tokio::spawn({
			let at = at.clone();
			let self_cloned = self.clone();

			async move {
				self_cloned.get_keys_paged_concurrent(prefix, at, get_keys_paged_tx).await.unwrap()
			}
		});

		let mut pairs = Vec::new();

		while let Some(keys) = get_keys_paged_rx.recv().await {
			// TODO: warning while batch empty
			let values = self
				.ws
				.batch::<Option<String>, _>(
					keys.iter().map(|key| state::get_storage_raw(key, at.as_ref())).collect(),
				)
				.await?;
			let keys_count = keys.len();
			let values_count = values.len();

			if keys_count != values_count {
				return Err(error::Node::KeyValuesCountMismatched {
					expect: keys_count,
					got: values_count,
				})?;
			}

			keys.into_iter().zip(values.into_iter()).for_each(|(k, v)| {
				if let Some(v) = v.result {
					pairs.push((k, v));
				} else {
					tracing::warn!("{k} has null value");
				}
			});

			let pairs_count = pairs.len();

			tracing::trace!("fetched {pairs_count} pairs");
		}

		Ok(pairs)
	}
}
