// std
use std::{sync::Arc, time::Instant};
// crates.io
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::sync::mpsc::{self, Sender};
// hack-ink
use crate::core::{
	error,
	jsonrpc::websocket::{Ws, WsInitializer},
	Result,
};
use subrpcer::state;
use substorager::StorageKey;

const PAGE_SIZE: usize = 512;
const KEY_LENGTH: usize = 64;

/// Start re-genesis process.
pub async fn run(uri: &str) -> Result<()> {
	let ws = Arc::new(WsInitializer::new().connect(uri).await?);
	let progresses = MultiProgress::new();
	let fetch_progress = progresses.add(ProgressBar::new(u64::MAX));
	let store_progress = progresses.insert_after(&fetch_progress, ProgressBar::new(u64::MAX));
	let start_time = Instant::now();
	let pairs =
		get_pairs_paged(ws.clone(), StorageKey::new(), None, fetch_progress, store_progress)
			.await?;

	progresses.clear().map_err(error::Generic::Io)?;

	println!("✓ fully exported {} pairs, takes {}s", pairs.len(), start_time.elapsed().as_secs());

	Ok(())
}

async fn get_pairs_paged(
	ws: Arc<Ws>,
	prefix: StorageKey,
	at: Option<()>,
	fetch_progress: ProgressBar,
	progress: ProgressBar,
) -> Result<Vec<(String, String)>> {
	let (get_keys_paged_tx, mut get_keys_paged_rx) = mpsc::channel(PAGE_SIZE);

	tokio::spawn({
		let ws = ws.clone();

		async move {
			get_keys_paged(ws.clone(), prefix, at, fetch_progress, get_keys_paged_tx).await.unwrap()
		}
	});

	let mut pairs = Vec::new();

	progress.set_style(
		ProgressStyle::with_template(
			"{spinner:.cyan} {elapsed:>9.yellow} 📂 stored  {pos:>8.cyan} {msg:.green}(...)",
		)
		.unwrap()
		.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
	);

	while let Some(keys) = get_keys_paged_rx.recv().await {
		let values = ws
			.batch::<Option<String>, _>(
				keys.iter().map(|key| state::get_storage_raw(key, at)).collect(),
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

		let progress_display_key = keys.get(0).map(ToOwned::to_owned).unwrap_or_default();

		keys.into_iter().zip(values.into_iter()).for_each(|(k, v)| {
			if let Some(v) = v.result {
				pairs.push((k, v));
			} else {
				tracing::warn!("[core::re_genesis] {k} has null value");
			}
		});

		{
			let pairs_count = pairs.len();

			tracing::trace!("stored {pairs_count} pairs");

			progress.set_position(pairs_count as _);
			progress.set_message(
				progress_display_key[..progress_display_key.len().min(KEY_LENGTH)].to_string(),
			);
		}
	}

	progress.set_length(progress.position());
	progress.finish();

	Ok(pairs)
}

async fn get_keys_paged(
	ws: Arc<Ws>,
	prefix: StorageKey,
	at: Option<()>,
	progress: ProgressBar,
	get_keys_paged_tx: Sender<Vec<String>>,
) -> Result<()> {
	let prefix = prefix.to_string();
	let mut start_key = None::<String>;
	let mut keys_count = 0_usize;
	// Debug.
	let mut i = 0;

	progress.set_style(
		ProgressStyle::with_template(
			"{spinner:.cyan} {elapsed:>9.yellow} 🔍 fetched {pos:>8.cyan} {msg:.green}(...)",
		)
		.unwrap()
		.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
	);

	loop {
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

		keys_count += downloaded_keys_count;

		if let Some(key) = downloaded_keys.last() {
			start_key = Some(key.to_owned());
		}

		{
			tracing::trace!("fetched  {} keys", keys_count);

			let key = start_key.clone().unwrap_or_default();

			progress.set_position(keys_count as _);
			progress.set_message(key[..key.len().min(KEY_LENGTH)].to_string());
		}

		get_keys_paged_tx.send(downloaded_keys).await.map_err(|_| error::Tokio::MpscSend)?;

		if downloaded_keys_count < PAGE_SIZE {
			progress.set_length(progress.position());
			progress.finish();

			return Ok(());
		}

		// Debug.
		if i < 5 {
			i += 1;
		} else {
			return Ok(());
		}
	}
}
