//! Export the state of Substrate-Base chain.

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
	jsonrpc::ws::Initializer,
	prelude::*,
	substrate_client::{BasicApi, Client},
	system,
};
use substorager::StorageKey;

/// Export configurations.
#[derive(Debug, Args)]
pub struct Config {
	/// Save the exported result to.
	///
	/// If the file is already exists, its content will be updated.
	#[clap(long, value_name = "PATH", default_value = "exported-state.json")]
	pub output: String,
	/// Timeout for the fetching.
	#[clap(long, value_name = "SECS", default_value = "5")]
	pub timeout: u64,
	/// Fetch all the data.
	///
	/// So, it conflicts with any other filter option.
	///
	/// Note:
	/// The default behaviour (without this option) is fetching according to metadata's pallet
	/// storage records, which means if there is any old storage prefix that can not be found in
	/// the current runtime's pallet storage names will be ignored.
	#[clap(verbatim_doc_comment, long, takes_value = false, conflicts_with_all = &["skip-pallets", "fork-off"])]
	pub all: bool,
	/// Skip these pallets, while fetching.
	///
	/// It's useful when you want to skip the 'large' pallets.
	#[clap(long, use_value_delimiter = true, value_name = "[PALLET_NAME]", conflicts_with = "all")]
	pub skip_pallets: Vec<String>,
	/// Fork off the chain.
	///
	/// Recommend to run with the `--output` option.
	///
	/// It will:
	/// - Skip `["System", "Babe", "Authorship", "Session", "Grandpa", "Beefy"]` pallets, but keep
	///   the `System::Account` data. (in order to make the new chain runnable)
	/// - Change the id and impl name to `*-export`.
	/// - Clear the bootnodes.
	/// - Set the `Staking::ForceEra` to `ForceNone`. (in order to prevent the validator set from
	///   changing mid-test)
	///
	/// Usually use this as below to get a runnable fork-off chain, and you can do whatever you
	/// want on it. Test new features, runtime upgrade, etc.
	/// ```sh
	/// # In general, dev chain's validator is `//Alice`, which is good for us to test locally.
	/// xxx-node --build-spec xxx-dev > xxx.json
	/// subalfred export-state wss://xxx --fork-off --skip-governance --output xxx.json --log subalfred::core::xxx-node
	/// xxx-node --chain xxx.json --alice --tmp
	/// ```
	#[clap(long, takes_value = false, conflicts_with = "all", verbatim_doc_comment)]
	pub fork_off: bool,
	/// Use `//Alice` to control the governance.
	///
	/// It's useful when you want to test the runtime upgrade.
	///
	/// It will:
	/// - Replace the sudo key with `//Alice`, if the pallet existed.
	/// - Replace the phragmen election and council members with `//Alice`, if the pallet existed.
	/// - Replace the technical membership and tech.comm members with `//Alice`, if the pallet
	///   existed.
	#[clap(long, takes_value = false, conflicts_with = "all", verbatim_doc_comment)]
	pub simple_governance: bool,
}

/// Start re-genesis process.
pub async fn run(uri: &str, at: Option<String>, config: &Config) -> Result<()> {
	let Config { timeout, all, skip_pallets, fork_off, .. } = config;
	let start_time = Instant::now();
	let client =
		Client::initialize(Initializer::new().request_timeout(Duration::from_secs(*timeout)), uri)
			.await?;
	let at = if at.is_some() { at } else { Some(client.get_finalized_head().await?) };
	let pairs = if *all {
		client.get_pairs_paged(StorageKey::new(), at).await?
	} else {
		let runtime_metadata =
			super::parse_raw_runtime_metadata(&client.get_runtime_metadata().await?)?;
		let mut pallets = runtime_metadata
			.pallets
			.iter()
			.filter_map(|pallet| pallet.storage.as_ref().map(|storage| storage.prefix.as_str()))
			.collect::<FxHashSet<_>>();

		skip_pallets.iter().for_each(|pallet| {
			pallets.remove(pallet.as_str());
		});

		let mut pairs = FxHashSet::default();

		if *fork_off {
			for (pallet, items) in vec![("System", vec!["Account"])].into_iter().chain(
				// runtime_metadata.pallets.iter().find_map(|pallet| {
				// 	if &pallet.name == "Staking" {
				// 		pallet.storage.as_ref().map(|storage| {
				// 			(
				// 				storage.prefix.as_str(),
				// 				storage
				// 					.entries
				// 					.iter()
				// 					.filter_map(|entry| {
				// 						let name = entry.name.as_str();
				//
				// 						if name == "Validators" || name == "Nominators" {
				// 							None
				// 						} else {
				// 							Some(name)
				// 						}
				// 					})
				// 					.collect(),
				// 			)
				// 		})
				// 	} else {
				// 		None
				// 	}
				// }),
				[],
			) {
				for item in items {
					tracing::trace!("fetching from {pallet}::{item}");

					client
						.get_pairs_paged(
							substorager::storage_key(pallet.as_bytes(), item.as_bytes()),
							at.clone(),
						)
						.await?
						.into_iter()
						.for_each(|pair| {
							pairs.insert(pair);
						});
				}
			}

			["System", "Babe", "Authorship", "Session", "Grandpa", "Beefy"].iter().for_each(
				|pallet| {
					pallets.remove(pallet);
				},
			);
		}

		for pallet in pallets {
			tracing::trace!("fetching from {pallet}");

			client
				.get_pairs_paged(
					StorageKey(subhasher::twox128(pallet.as_bytes()).to_vec()),
					at.clone(),
				)
				.await?
				.into_iter()
				.for_each(|pair| {
					pairs.insert(pair);
				});
		}

		pairs.into_iter().collect()
	};
	let pairs_count = pairs.len();

	store(pairs, config)?;

	println!("✓ fully exported {pairs_count} pairs, takes {}s", start_time.elapsed().as_secs());

	Ok(())
}

fn store(pairs: Vec<(String, String)>, config: &Config) -> Result<()> {
	let Config { output, fork_off, simple_governance, .. } = config;
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

	// Use a different id and impl name.
	json["id"] = Value::String(format!("{}-export", json["id"].as_str().unwrap_or_default()));
	json["name"] = Value::String(format!("{}-export", json["name"].as_str().unwrap_or_default()));
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

	pairs.into_iter().for_each(|(k, v)| {
		top.insert(k, Value::String(v));
	});

	if *fork_off {
		let staking_force_era = substorager::storage_key(b"Staking", b"ForceEra").to_string();

		top.insert(staking_force_era, Value::String("0x02".into()));
	}
	if *simple_governance {
		let alice = Value::String(
			"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".into(),
		);
		let alice_members = Value::String(
			"0x04d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".into(),
		);
		let alice_phragmen_election = Value::String("0x04d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0010a5d4e800000000000000000000000010a5d4e80000000000000000000000".into());
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
