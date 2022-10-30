//! Export core library.

// std
use std::{
	path::Path,
	time::{Duration, Instant},
};
// crates.io
use fxhash::FxHashSet;
// hack-ink
use super::{fork_off, ChainSpec, ForkOffConfig};
use crate::{
	jsonrpc::ws::Initializer,
	node,
	prelude::*,
	substrate_client::{Apis, Client},
	system,
};
use substorager::StorageKey;

/// Export the chain state from a node's WS RPC endpoint at a specific block height.
///
/// If `at` is `None`, this will start from the latest block.
/// Use `skip_pallets` to skip the exporting of some pallets, we usually use this when the pallet's
/// data is too large.
pub async fn export(
	uri: &str,
	at: Option<String>,
	timeout: u32,
	all: bool,
	skip_pallets: &[String],
	fork_off_config: &ForkOffConfig,
) -> Result<()> {
	let ForkOffConfig { renew_consensus_with, simple_governance, disable_default_bootnodes } =
		fork_off_config;
	let start_time = Instant::now();
	let client = Client::initialize(
		Initializer::new().request_timeout(Duration::from_secs(timeout as _)),
		uri,
	)
	.await?;
	let at = if at.is_some() { at } else { Some(client.get_finalized_head().await?) };
	let pairs = if all {
		client.get_pairs_paged(StorageKey::new(), at).await?
	} else {
		let runtime_metadata =
			node::parse_raw_runtime_metadata(&client.get_runtime_metadata().await?)?;
		let mut pallets = runtime_metadata
			.pallets
			.iter()
			.filter_map(|pallet| pallet.storage.as_ref().map(|storage| storage.prefix.as_str()))
			.collect::<FxHashSet<_>>();

		skip_pallets.iter().for_each(|pallet| {
			pallets.remove(pallet.as_str());
		});

		let mut pairs = FxHashSet::default();

		if renew_consensus_with.is_some() {
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
	let path = renew_consensus_with.to_owned().unwrap_or_else(|| "default-chain-spec.json".into());
	let path = Path::new(&path);
	let mut chain_spec = if path.is_file() {
		system::read_file_to_struct::<_, ChainSpec>(path)?
	} else {
		ChainSpec::default()
	};

	// Use a different id and impl name.
	chain_spec.id = format!("{}-export", chain_spec.id);
	chain_spec.name = format!("{}-export", chain_spec.name);

	if *disable_default_bootnodes {
		chain_spec.boot_nodes.clear();
	}

	let top = &mut chain_spec.genesis.raw.top;

	pairs.into_iter().for_each(|(k, v)| {
		top.insert(k, v);
	});

	if renew_consensus_with.is_some() {
		let staking_force_era = substorager::storage_key(b"Staking", b"ForceEra").to_string();

		top.insert(staking_force_era, "0x02".into());
	}
	if *simple_governance {
		fork_off::set_simple_governance(&mut chain_spec);
	}

	super::write_to_custom_extension_file(path, "export", chain_spec)?;

	println!("✓ fully exported {pairs_count} pairs, takes {}s", start_time.elapsed().as_secs());

	Ok(())
}
