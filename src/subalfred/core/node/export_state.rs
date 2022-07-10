// std
use std::{
	path::Path,
	time::{Duration, Instant},
};
// crates.io
use clap::Args;
use fxhash::FxHashSet;
use serde_json::Value;
// hack-ink
use crate::core::{
	error,
	jsonrpc::ws::Initializer,
	substrate_client::{Api, Client},
	system, Result,
};
use substorager::StorageKey;

/// Export configurations.
#[derive(Debug, Args)]
pub struct Config {
	/// Save the exported result to.
	#[clap(long, value_name = "PATH", default_value = "exported-state.json")]
	pub output: String,
	/// Fetch all the data.
	///
	/// Note:
	/// The default behaviour is fetching according to metadata's pallet storage records,
	/// which means if there is any old storage prefix that can not be found in the current
	/// runtime's pallet storage names will be ignored.
	#[clap(long, takes_value = false)]
	pub all: bool,
	// pub pallets: Vec<String>,
	/// TODO:doc
	#[clap(long, use_value_delimiter = true, value_name = "[PALLET_NAME]")]
	pub skip_pallets: Vec<String>,
	/// Skip exporting the authority related storages.
	#[clap(long, takes_value = false)]
	pub skip_authority: bool,
	/// Skip exporting the collective and sudo related storages.
	#[clap(long, takes_value = false)]
	pub skip_collective: bool,
}

/// Start re-genesis process.
pub async fn run(uri: &str, at: Option<String>, config: &Config) -> Result<()> {
	let start_time = Instant::now();
	let client =
		Client::initialize(Initializer::new().request_timeout(Duration::from_secs(600)), uri)
			.await?;
	let at = if at.is_some() { at } else { Some(client.get_finalized_head().await?) };
	let Config { all, skip_pallets, .. } = config;
	let pairs = if *all {
		client.get_pairs_paged(StorageKey::new(), at).await?
	} else {
		let pallets = super::parse_raw_runtime_metadata(&client.get_runtime_metadata().await?)?
			.pallets
			.into_iter()
			.filter_map(|pallet| pallet.storage.map(|_| pallet.name))
			.collect::<FxHashSet<_>>();
		let skip_pallets = skip_pallets.iter().cloned().collect::<FxHashSet<_>>();
		let filtered_pallets = pallets.difference(&skip_pallets);
		let mut pairs = Vec::new();

		for pallet in filtered_pallets {
			tracing::trace!("fetching from {pallet}");

			let mut fetched_pairs = client
				.get_pairs_paged(
					StorageKey(subhasher::twox128(pallet.as_bytes()).to_vec()),
					at.clone(),
				)
				.await?;

			pairs.append(&mut fetched_pairs);
		}

		pairs
	};
	let pairs_count = pairs.len();

	store(pairs, &config)?;

	println!("✓ fully exported {pairs_count} pairs, takes {}s", start_time.elapsed().as_secs());

	Ok(())
}

fn store(pairs: Vec<(String, String)>, config: &Config) -> Result<()> {
	let Config { output, skip_authority, skip_collective, .. } = config;
	let path = Path::new(output);
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
		let mut v = Vec::new();

		if *skip_authority {
			[b"Babe".as_slice(), b"Authorship", b"Session", b"Grandpa", b"Beefy"]
				.iter()
				.for_each(|s| v.push(array_bytes::bytes2hex("0x", &subhasher::twox128(s))));
			[(b"Staking".as_slice(), b"Validators".as_slice()), (b"Staking", b"Nominators")]
				.iter()
				.for_each(|(p, i)| {
					v.push(array_bytes::bytes2hex("0x", &substorager::storage_key(p, i)))
				});
		}
		// if *skip_collective {
		// 	[
		// 		b"Council".as_slice(),
		// 		b"TechnicalCommittee",
		// 		b"PhragmenElection",
		// 		b"TechnicalMembership",
		// 		b"Sudo",
		// 	]
		// 	.iter()
		// 	.for_each(|s| v.push(array_bytes::bytes2hex("0x", &subhasher::twox128(s))));
		// }

		v
	};

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
	if *skip_collective {
		let alice = Value::String(
			"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".into(),
		);
		let alice_members = Value::String(
			"0x04d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".into(),
		);
		let alice_phragmen_election =
	Value::String("
	0x04d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0010a5d4e800000000000000000000000010a5d4e80000000000000000000000"
	.into());

		let council = substorager::storage_key(b"Council", b"Members").to_string();
		let technical_committee =
			substorager::storage_key(b"TechnicalCommittee", b"Members").to_string();
		let phragmen_election =
			substorager::storage_key(b"PhragmenElection", b"Members").to_string();
		let technical_membership =
			substorager::storage_key(b"TechnicalMembership", b"Members").to_string();
		let sudo = substorager::storage_key(b"Sudo", b"Key").to_string();

		top.insert(council, alice_members.clone());
		top.insert(technical_committee, alice_members.clone());
		top.insert(technical_membership, alice_members);
		top.insert(phragmen_election, alice_phragmen_election);
		top.insert(sudo, alice);
	}

	system::write_data_to_file(path, &serde_json::to_vec(&json).map_err(error::Generic::Serde)?)?;

	Ok(())
}
