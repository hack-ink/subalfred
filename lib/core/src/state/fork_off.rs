//! Fork-off core library.

// std
use std::{mem, path::Path};
// crates.io
#[cfg(feature = "clap")] use clap::Args;
use fxhash::FxHashMap;
// hack-ink
use super::ChainSpec;
use crate::{prelude::*, system};

/// Fork-off configurations.
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug)]
pub struct ForkOffConfig {
	/// Renew the consensus relate things of the chain.
	///
	/// We need the dev chain specification to renew the consensus relates genesis. Otherwise, the
	/// fork-off chain won't produce block.
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
	///
	/// ```sh
	/// xxx-node --export-state > xxx-export.json
	/// xxx-node --build-spec xxx-dev > xxx-dev.json
	/// subalfred state fork-off xxx-export.json --renew-consensus-with xxx.dev.json --simple-governance --disable-default-bootnodes
	/// xxx-node --chain xxx.json.fork-off --alice --tmp
	/// ```
	///
	/// Note:
	/// `--alice` only works for which dev chain's genesis validator is `//Alice`, otherwise the
	/// new chain won't produce block. If your dev chain's genesis validator is `//Bob`, then
	/// running with `--bob`. But if your dev chain's genesis validator isn't any one of the
	/// well-known keys, then you should start the node with `--validator` and insert the key
	/// manually.
	#[cfg_attr(
		feature = "clap",
		arg(verbatim_doc_comment, long, value_name = "PATH", conflicts_with = "all")
	)]
	pub renew_consensus_with: Option<String>,
	/// Use `//Alice` to control the governance.
	///
	/// It's useful when you want to test the runtime upgrade.
	///
	/// It will:
	/// - Replace sudo key with `//Alice`, if the sudo pallet existed.
	/// - Replace phragmen election and council members with `//Alice`, if the collective pallet existed.
	/// - Replace technical membership and tech.comm members with `//Alice`, if the membership pallet
	///   existed.
	#[cfg_attr(
		feature = "clap",
		arg(verbatim_doc_comment, long, conflicts_with = "all")
	)]
	pub simple_governance: bool,
	/// Disable adding the default bootnodes to the specification.
	#[cfg_attr(feature = "clap", arg(verbatim_doc_comment, long))]
	pub disable_default_bootnodes: bool,
}

/// Fork-off the state with the specific configurations.
pub fn fork_off<P>(target_chain_spec_path: P, config: &ForkOffConfig) -> Result<()>
where
	P: AsRef<Path>,
{
	subalfred_util::execution_timer!("state::fork-off");

	let target_chain_spec_path = target_chain_spec_path.as_ref();
	let ForkOffConfig { renew_consensus_with, simple_governance, disable_default_bootnodes } =
		config;
	let mut chain_spec = if let Some(renew_consensus_with) = renew_consensus_with {
		let (mut target_chain_spec, dev_chain_spec) =
			super::read_chain_spec_concurrent(target_chain_spec_path, renew_consensus_with)?;

		clear_consensus(&mut target_chain_spec);

		super::override_top(dev_chain_spec, target_chain_spec)
	} else {
		system::read_file_to_struct::<_, ChainSpec>(target_chain_spec_path)?
	};

	if *simple_governance {
		set_simple_governance(&mut chain_spec);
	}
	if *disable_default_bootnodes {
		chain_spec.boot_nodes.clear();
	}

	super::write_to_custom_extension_file(target_chain_spec_path, "fork-off", chain_spec)
}

fn clear_consensus(chain_spec: &mut ChainSpec) {
	let top = &mut chain_spec.genesis.raw.top;
	let system_prefix = array_bytes::bytes2hex("0x", &subhasher::twox128(b"System"));
	let system_account_prefix =
		array_bytes::bytes2hex("0x", &substorager::storage_key(b"System", b"Account"));
	// TODO: if the `top` is sorted, we can pop the prefix while it is passed
	let ignore_prefixes = [b"Babe".as_ref(), b"Authorship", b"Session", b"Grandpa", b"Beefy"]
		.iter()
		.map(|prefix| array_bytes::bytes2hex("0x", &subhasher::twox128(prefix)))
		.collect::<Vec<_>>();
	// TODO: use `BTreeMap` for `top`, sortable
	let mut new_top = FxHashMap::default();

	mem::swap(top, &mut new_top);

	*top = new_top
		.into_iter()
		.filter_map(|(k, v)| {
			if k.starts_with(&system_prefix) {
				if k.starts_with(&system_account_prefix) {
					Some((k, v))
				} else {
					None
				}
			} else if ignore_prefixes.iter().any(|prefix| k.starts_with(prefix)) {
				None
			} else {
				Some((k, v))
			}
		})
		.collect();

	top.insert(substorager::storage_key(b"Staking", b"ForceEra").to_string(), "0x02".into());
	top.remove(&substorager::storage_key(b"System", b"LastRuntimeUpgrade").to_string());
}

pub(super) fn set_simple_governance(chain_spec: &mut ChainSpec) {
	let top = &mut chain_spec.genesis.raw.top;
	let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
	let alice_members = "0x04d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
	// TODO: this might be different on different chain
	let alice_phragmen_election = "0x04d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0010a5d4e800000000000000000000000010a5d4e80000000000000000000000";
	let council = substorager::storage_key(b"Council", b"Members");
	let technical_committee = substorager::storage_key(b"TechnicalCommittee", b"Members");
	let phragmen_election = substorager::storage_key(b"PhragmenElection", b"Members");
	let technical_membership = substorager::storage_key(b"TechnicalMembership", b"Members");
	let sudo = substorager::storage_key(b"Sudo", b"Key");

	// TODO: skip if not exist
	top.insert(council.to_string(), alice_members.into());
	top.insert(technical_committee.to_string(), alice_members.into());
	top.insert(technical_membership.to_string(), alice_members.into());
	top.insert(phragmen_election.to_string(), alice_phragmen_election.into());
	top.insert(sudo.to_string(), alice.into());
}
