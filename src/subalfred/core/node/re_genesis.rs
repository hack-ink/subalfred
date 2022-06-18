// crates.io
use console::Style;
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
const BATCH_SIZE: usize = 512;

/// Start re-genesis process.
pub async fn run(uri: &str) -> Result<()> {
	let ws = WsInitializer::new().connect(uri).await?;
	let progresses = MultiProgress::new();
	let req_progress = progresses.add(ProgressBar::new(u64::MAX));
	let bat_progress = progresses.insert_after(&req_progress, ProgressBar::new(u64::MAX));
	let req_style = ProgressStyle::with_template("{spinner:.cyan} {elapsed:.yellow} {msg}")
		.unwrap()
		.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
	let bat_style = ProgressStyle::with_template("{spinner:.cyan} {elapsed:.yellow} {msg}")
		.unwrap()
		.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
	let (req_tx, mut req_rx) = mpsc::channel::<(String, usize)>(PAGE_SIZE);
	let (bat_tx, mut bat_rx) = mpsc::channel::<(String, usize)>(PAGE_SIZE);

	req_progress.set_style(req_style);
	bat_progress.set_style(bat_style);

	tokio::spawn(async move {
		let green = Style::new().green();
		let cyan = Style::new().cyan();

		loop {
			if let Some((key, keys_len)) = req_rx.recv().await {
				req_progress.set_position(keys_len as _);
				req_progress.set_message(format!(
					"{} {} {} {}",
					green.apply_to("Fetched"),
					cyan.apply_to(req_progress.position()),
					green.apply_to("keys, current at"),
					cyan.apply_to(&key[..key.len().min(16)])
				));
				bat_tx.send((key, keys_len)).await;
			} else {
				req_progress.finish_with_message(format!(
					"{} {} {}",
					green.apply_to("Fetched"),
					cyan.apply_to(req_progress.position()),
					green.apply_to("keys"),
				));

				break;
			}
		}
	});
	tokio::spawn(async move {
		let green = Style::new().green();
		let cyan = Style::new().cyan();

		loop {
			if let Some((key, keys_len)) = bat_rx.recv().await {
				bat_progress.set_position(keys_len as _);
				bat_progress.set_message(format!(
					"{} {} {} {}",
					green.apply_to("Stored"),
					cyan.apply_to(bat_progress.position()),
					green.apply_to("pairs, current at"),
					cyan.apply_to(&key[..key.len().min(16)])
				));
			} else {
				bat_progress.set_length(bat_progress.position());
				bat_progress.finish_with_message(format!(
					"{} {} {}",
					green.apply_to("Stored"),
					cyan.apply_to(bat_progress.position()),
					green.apply_to("pairs"),
				));

				break;
			}
		}
	});

	get_pairs_paged(&ws, StorageKey::new(), None, req_tx).await?;

	Ok(())
}

async fn get_pairs_paged(
	ws: &Ws,
	prefix: StorageKey,
	at: Option<()>,
	req_tx: Sender<(String, usize)>,
) -> Result<Vec<(String, String)>> {
	let keys = get_keys_paged(ws, prefix, at, req_tx).await?;
	let mut pairs = Vec::new();

	for keys in keys.chunks(BATCH_SIZE) {
		let values = ws
			.batch::<Option<String>, _>(
				keys.iter().map(|key| state::get_storage_raw(key, at)).collect(),
			)
			.await?;
		let keys_len = keys.len();
		let values_len = values.len();

		if keys_len != values_len {
			return Err(error::Node::KeyValuesCountMismatched {
				expect: keys_len,
				got: values_len,
			})?;
		}

		keys.iter().cloned().zip(values.into_iter()).for_each(|(k, v)| {
			if let Some(v) = v.result {
				pairs.push((k, v));
			} else {
				tracing::warn!("[core::re_genesis] {k} has null value");
			}
		});

		tracing::trace!("Stored {} pairs.", pairs.len());
	}

	// progresses.clear().unwrap();

	Ok(pairs)
}

async fn get_keys_paged(
	ws: &Ws,
	prefix: StorageKey,
	at: Option<()>,
	req_tx: Sender<(String, usize)>,
) -> Result<Vec<String>> {
	let prefix = prefix.to_string();
	let mut start_key = None::<String>;
	let mut keys = Vec::new();
	// Debug.
	// let mut i = 0;

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

		keys.extend(downloaded_keys);

		let keys_len = keys.len();

		req_tx.send((start_key.clone().unwrap_or_default(), keys_len)).await;

		if downloaded_keys_count < PAGE_SIZE {
			tracing::trace!("Downloaded {} keys.", keys_len);

			break;
		}

		if let Some(key) = keys.last() {
			start_key = Some(key.to_owned());
		}

		tracing::trace!("Downloaded {} keys.", keys_len);

		// Debug.
		// if i < 5 {
		// 	i += 1;
		// } else {
		// 	break;
		// }
	}

	Ok(keys)
}
