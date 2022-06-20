// std
use std::{path::Path, sync::Arc, time::Instant};
// crates.io
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use serde_json::Value;
use tokio::sync::mpsc::{self, Sender};
// hack-ink
use super::E_INVALID_PROGRESS_BAR_TEMPLATE;
use crate::core::{
	error,
	jsonrpc::websocket::{Ws, WsInitializer},
	system, Result,
};
use subrpcer::{chain, state};
use substorager::StorageKey;

static PROGRESSES: Lazy<(MultiProgress, ProgressBar, ProgressBar)> = Lazy::new(|| {
	let progresses = MultiProgress::new();
	let fetch_progress = progresses.add(ProgressBar::new(u64::MAX));
	let store_progress = progresses.insert_after(&fetch_progress, ProgressBar::new(u64::MAX));

	fetch_progress.set_style(
		ProgressStyle::with_template(
			"{spinner:.cyan} {elapsed:>9.yellow} 🔍 fetched {pos:>8.cyan} {msg:.green}(...)",
		)
		.expect(E_INVALID_PROGRESS_BAR_TEMPLATE)
		.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
	);
	store_progress.set_style(
		ProgressStyle::with_template(
			"{spinner:.cyan} {elapsed:>9.yellow} 📂  stored {pos:>8.cyan} {msg:.green}(...)",
		)
		.expect(E_INVALID_PROGRESS_BAR_TEMPLATE)
		.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
	);

	(progresses, fetch_progress, store_progress)
});

const PAGE_SIZE: usize = 512;
const KEY_LENGTH: usize = 64;

/// Export configurations.
#[derive(Debug)]
pub struct ExportConfig {
	/// Save the exported result to.
	pub path: String,
	/// Skip exporting the authority related storages.
	pub skip_authority: bool,
	/// Skip exporting the collective and sudo related storages.
	pub skip_collective: bool,
	// TODO
	// pub renew_runtime_code: bool,
}

/// Start re-genesis process.
pub async fn run(uri: &str, at: Option<String>, config: ExportConfig) -> Result<()> {
	let ws = Arc::new(WsInitializer::new().connect(uri).await?);
	let at = if at.is_some() {
		at
	} else {
		Some(ws.request::<String, _>(chain::get_finalized_head_raw()).await?.result)
	};
	let start_time = Instant::now();
	let pairs = get_pairs_paged(ws.clone(), StorageKey::new(), at).await?;

	PROGRESSES.0.clear().map_err(error::Generic::Io)?;

	println!("✓ fully exported {} pairs, takes {}s", pairs.len(), start_time.elapsed().as_secs());

	dump_to_json(pairs, &config)?;

	Ok(())
}

async fn get_pairs_paged(
	ws: Arc<Ws>,
	prefix: StorageKey,
	at: Option<String>,
) -> Result<Vec<(String, String)>> {
	let (get_keys_paged_tx, mut get_keys_paged_rx) = mpsc::channel(PAGE_SIZE);

	tokio::spawn({
		let ws = ws.clone();
		let at = at.clone();

		async move { get_keys_paged(ws, prefix, at, get_keys_paged_tx).await.unwrap() }
	});

	let mut pairs = Vec::new();
	let progress = &PROGRESSES.2;

	while let Some(keys) = get_keys_paged_rx.recv().await {
		let values = ws
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
	at: Option<String>,
	get_keys_paged_tx: Sender<Vec<String>>,
) -> Result<()> {
	let prefix = prefix.to_string();
	let mut start_key = None::<String>;
	let mut keys_count = 0;
	let progress = &PROGRESSES.1;

	// Debug.
	// let mut i = 0;

	loop {
		let response = ws
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

		{
			tracing::trace!("fetched {keys_count} keys");

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
		// if i < 5 {
		// 	i += 1;
		// } else {
		// 	return Ok(());
		// }
	}
}

// TODO: async
fn dump_to_json(pairs: Vec<(String, String)>, config: &ExportConfig) -> Result<()> {
	let ExportConfig { path, skip_authority, skip_collective } = config;
	let path = Path::new(path);
	let mut json = if path.is_file() {
		serde_json::from_slice(&system::read_file_to_vec(path)?).map_err(error::Generic::Serde)?
	} else {
		serde_json::json!({
			"genesis": {
				"raw": {
					"top": {}
				}
			}
		})
	};

	// Use a different spec name.
	json["name"] = Value::String(format!("{}-export", json["name"].as_str().unwrap_or_default()));
	json["id"] = Value::String(format!("{}-export", json["id"].as_str().unwrap_or_default()));
	// Clear the bootnodes.
	json["bootNodes"] = Value::Array(Vec::new());

	let top = json
		.get_mut("genesis")
		.ok_or(error::Node::InvalidSpecificationFile)?
		.get_mut("raw")
		.ok_or(error::Node::InvalidSpecificationFile)?
		.get_mut("top")
		.ok_or(error::Node::InvalidSpecificationFile)?
		.as_object_mut()
		.ok_or(error::Node::InvalidSpecificationFile)?;
	let skip_storages = {
		let mut s = Vec::new();

		if *skip_authority {
			s.extend_from_slice(&[
				b"Session".as_slice(),
				b"Babe",
				b"Grandpa",
				b"FinalityTracker",
				b"Authorship",
			]);
		}
		if *skip_collective {
			s.extend_from_slice(&[
				b"Sudo".as_slice(),
				b"PhragmenElection",
				b"Council",
				b"TechnicalMembership",
				b"TechnicalCommittee",
			]);
		}

		s
	}
	.into_iter()
	.map(|s| array_bytes::bytes2hex("0x", &subhasher::twox128(s)))
	.collect::<Vec<_>>();

	pairs.into_iter().for_each(|(k, v)| {
		// TODO:
		// time complexity is O(n^2)
		// this algorithm is a shit
		// we can read the storage prefixes from metadata
		if !skip_storages.iter().any(|p| k.starts_with(p)) {
			top.insert(k, Value::String(v));
		}
	});

	if *skip_authority {
		let system_last_runtime_upgrade =
			substorager::storage_key(b"System", b"LastRuntimeUpgrade").to_string();
		let staking_force_era = substorager::storage_key(b"Staking", b"ForceEra").to_string();

		let _ = top.remove(&system_last_runtime_upgrade);
		top.insert(staking_force_era, Value::String("0x02".into()));
	}

	system::write_data_to_file(path, &serde_json::to_vec(&json).map_err(error::Generic::Serde)?)?;

	Ok(())
}