// hack-ink
use crate::core::{
	jsonrpc::websocket::{Ws, WsInitializer},
	Result,
};
use subrpcer::state;
use substorager::StorageKey;

const PAGE_SIZE: usize = 512;
const BATCH_SIZE: usize = 512;

/// TODO
pub async fn run(uri: &str) -> Result<()> {
	let ws = WsInitializer::new().connect(uri).await?;

	get_pairs_paged(&ws, StorageKey::new(), None).await?;

	Ok(())
}

async fn get_pairs_paged(ws: &Ws, prefix: StorageKey, at: Option<()>) -> Result<()> {
	let keys = get_keys_paged(ws, prefix, at).await?;

	for chunk_keys in keys.chunks(BATCH_SIZE) {
		ws.batch::<String, _>(
			chunk_keys.iter().cloned().map(|key| state::get_storage_raw(key, at)).collect(),
		)
		.await?;
	}

	Ok(())
}

async fn get_keys_paged(ws: &Ws, prefix: StorageKey, at: Option<()>) -> Result<Vec<String>> {
	let prefix = prefix.to_string();
	let mut start_key = None::<String>;
	let mut keys = Vec::new();

	loop {
		tracing::info!("Downloaded {} keys.", keys.len());

		// TODO: does the `downloaded_keys` contains `start_key`
		let response = ws
			.request::<Vec<String>, _>(state::get_keys_paged_raw(
				&prefix,
				PAGE_SIZE,
				start_key.as_ref(),
				at,
			))
			.await?;
		let downloaded_keys = response.result;
		let downloaded_keys_count = downloaded_keys.len();

		keys.extend(downloaded_keys);

		// Debug.
		break;

		if downloaded_keys_count < PAGE_SIZE {
			tracing::info!("Downloaded {} keys.", keys.len());

			break;
		}

		if let Some(key) = keys.last() {
			start_key = Some(key.to_owned());
		}
	}

	Ok(keys)
}
