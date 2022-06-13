// hack-ink
use crate::core::{http, Result};
use subrpcer::state;
use substorager::StorageKey;

const COUNT: u16 = 512;

pub async fn run(uri: &str) -> Result<()> {
	get_pairs_paged(uri, StorageKey::new()).await?;

	Ok(())
}

async fn get_pairs_paged(uri: &str, prefix: StorageKey) -> Result<()> {
	let prefix = prefix.to_string();
	let mut start_key = None::<String>;
	let mut keys = Vec::new();

	loop {
		tracing::info!("Downloaded {} keys.", keys.len());

		// TODO: does the `downloaded_keys` contains `start_key`
		let response = http::send_jsonrpc::<_, Vec<String>>(
			uri,
			&state::get_keys_paged_once(&prefix, COUNT, start_key.as_ref(), None::<()>),
		)
		.await?;
		let downloaded_keys = response.result;
		let downloaded_keys_count = downloaded_keys.len() as u16;

		keys.extend(downloaded_keys);

		if downloaded_keys_count < COUNT {
			tracing::info!("Downloaded {} keys.", keys.len());

			break;
		}

		if let Some(key) = keys.last() {
			start_key = Some(key.to_owned());
		}
	}

	Ok(())
}
