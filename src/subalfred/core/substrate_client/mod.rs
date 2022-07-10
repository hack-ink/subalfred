//! Substrate-Like API client implementation.

// TODO: maybe HTTP

mod api;
pub use api::Api;

// std
use std::sync::Arc;
// crates.io
use tokio::sync::{
	mpsc::{self, Sender},
	Mutex,
};
// subalfred
use crate::core::{
	error,
	jsonrpc::ws::{Initializer, Ws},
	Result,
};
use subrpcer::{chain, state};
use substorager::StorageKey;

type TxId = u8;

const GET_KEYS_PAGED_TX_ID: TxId = 0;
const PAGE_SIZE: usize = 512;

const E_GET_KEYS_PAGED_TX_NOT_FOUND: &str =
	"[core::substrate_client] `GET_KEYS_PAGED_TX` not found";

/// Substrate-Like API websocket client.
#[derive(Clone)]
pub struct Client {
	/// Websocket connection.
	ws: Arc<Ws>,
	/// Some txs which use for organizing the tasks to speed up the async process.
	txs: Arc<Mutex<Vec<(TxId, Tx)>>>,
}
impl Client {
	/// TODO: doc
	pub async fn initialize(initializer: Initializer, uri: &str) -> Result<Self> {
		Ok(Self {
			ws: Arc::new(initializer.connect(uri).await?),
			txs: Arc::new(Mutex::new(Vec::new())),
		})
	}
}
#[async_trait::async_trait]
impl Api for Client {
	async fn get_finalized_head(&self) -> Result<String> {
		Ok(self.ws.request::<String, _>(chain::get_finalized_head_raw()).await?.result)
	}

	async fn get_runtime_metadata(&self) -> Result<String> {
		Ok(self.ws.request::<String, _>(state::get_metadata_raw()).await?.result)
	}

	async fn get_pairs_paged(
		&self,
		prefix: StorageKey,
		at: Option<String>,
	) -> Result<Vec<(String, String)>> {
		let (get_keys_paged_tx, mut get_keys_paged_rx) = mpsc::channel(PAGE_SIZE);

		// FIXME: multiple id
		self.txs.lock().await.push((GET_KEYS_PAGED_TX_ID, Tx::Strings(get_keys_paged_tx)));

		tokio::spawn({
			let at = at.clone();
			let self_cloned = self.clone();

			async move { self_cloned.get_keys_paged(prefix, at).await.unwrap() }
		});

		let mut pairs = Vec::new();

		while let Some(keys) = get_keys_paged_rx.recv().await {
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

	async fn get_keys_paged(&self, prefix: StorageKey, at: Option<String>) -> Result<Vec<String>> {
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
			}

			tracing::trace!("fetched {keys_count} keys");

			self.txs
				.lock()
				.await
				.iter()
				.find_map(get_tx_as(GET_KEYS_PAGED_TX_ID, Tx::as_strings_tx))
				.ok_or(error::almost_impossible(E_GET_KEYS_PAGED_TX_NOT_FOUND))?
				.send(downloaded_keys)
				.await
				.map_err(|_| error::Tokio::MpscSend)?;

			if downloaded_keys_count < PAGE_SIZE {
				let mut txs = self.txs.lock().await;

				if let Some(i) = txs.iter().position(|(tx_id, _)| tx_id == &GET_KEYS_PAGED_TX_ID) {
					txs.remove(i);
				}

				// The result is useless, when the `Client::txs` is enabled.
				// Because in every single loop,
				// `GET_KEYS_PAGED_TX` will yield the result to outside.
				return Ok(Vec::new());
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

enum Tx {
	Strings(Sender<Vec<String>>),
}
impl Tx {
	fn as_strings_tx(&self) -> Option<&Sender<Vec<String>>> {
		match self {
			Self::Strings(tx) => Some(tx),
		}
	}
}

fn get_tx_as<F, T>(tx_id: TxId, mut tx_as: F) -> impl FnMut(&(TxId, Tx)) -> Option<&Sender<T>>
where
	F: FnMut(&Tx) -> Option<&Sender<T>>,
{
	move |(tx_id_, tx)| if tx_id_ == &tx_id { tx_as(tx) } else { None }
}
