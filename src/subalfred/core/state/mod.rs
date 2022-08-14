//! A set of tools to process Substrate-like node state.

// std
use std::{path::Path, thread};
// crates.io
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// hack-ink
use crate::core::{prelude::*, system};

// TODO: doc & move this to substrate-minimal
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChainSpec {
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub id: String,
	#[serde(default)]
	pub chain_type: String,
	#[serde(default)]
	pub boot_nodes: Vec<String>,
	pub telemetry_endpoints: Option<String>,
	pub protocol_id: Option<String>,
	pub fork_id: Option<String>,
	pub properties: Option<Value>,
	#[serde(default)]
	pub extensions: Value,
	#[serde(default)]
	pub genesis: Genesis,
	#[serde(default)]
	pub code_substitutes: Value,
}
// TODO: doc & move this to substrate-minimal
#[allow(missing_docs)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Genesis {
	#[serde(default)]
	pub raw: Raw,
}
// TODO: doc & move this to substrate-minimal
#[allow(missing_docs)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Raw {
	#[serde(default)]
	pub top: FxHashMap<String, String>,
	#[serde(default)]
	pub children_default: Value,
}

fn read_chain_spec_concurrent<P>(a: P, b: P) -> Result<(ChainSpec, ChainSpec)>
where
	P: Send + AsRef<Path>,
{
	let (a, b) = thread::scope(|scope| {
		let a = scope.spawn(|| system::read_file_to_struct::<_, ChainSpec>(a));
		let b = scope.spawn(|| system::read_file_to_struct::<_, ChainSpec>(b));

		(a.join(), b.join())
	});

	Ok((a.map_err(error::quick_err)??, b.map_err(error::quick_err)??))
}

/// Check the diff between two states.
///
/// Note:
/// This is not a symmetric diff.
/// `a.diff(b)` may equals to `b.diff(a)`, but not always.
pub fn diff<P>(a: P, b: P) -> Result<Vec<String>>
where
	P: Send + AsRef<Path>,
{
	crate::execution_timer!("diff state");

	if a.as_ref() == b.as_ref() {
		return Ok(Vec::new());
	}

	let (a, b) = read_chain_spec_concurrent(a, b)?;
	let (a, mut b) = (a.genesis.raw.top, b.genesis.raw.top);
	let mut diff = Vec::new();

	for (a_k, a_v) in a {
		if let Some(b_v) = b.remove(&a_k) {
			// Different value under the same key.
			if a_v != b_v {
				diff.push(format!("-{a_k}:{a_v}\n+{a_k}:{b_v}"));
			}

		// Completely same.
		}
		// The keys only appear in a.
		else {
			diff.push(format!("-{a_k}:{a_v}"));
		}
	}
	// The keys only appear in b.
	for (k, v) in b {
		diff.push(format!("+{k}:{v}"));
	}

	Ok(diff)
}

/// Override state a with b.
pub fn r#override<P>(a: P, b: P) -> Result<()>
where
	P: Send + AsRef<Path>,
{
	crate::execution_timer!("override state");

	let (a, b) = (a.as_ref(), b.as_ref());

	if a == b {
		return Ok(());
	}

	let (mut a_spec, b_spec) = read_chain_spec_concurrent(a, b)?;
	let (a_state, mut b_state) = (&mut a_spec.genesis.raw.top, b_spec.genesis.raw.top);

	for (a_k, a_v) in a_state.iter_mut() {
		if let Some(b_v) = b_state.remove(a_k) {
			// Different value under the same key.
			if a_v != &b_v {
				*a_v = b_v;
			}

			// Completely same.
		}

		// The keys only appear in a.
	}
	// The keys only appear in b.
	for (k, v) in b_state {
		a_state.insert(k, v);
	}

	system::write_data_to_file(
		a.with_file_name(format!("{}.merge", a.file_name().expect("[core::state] able to read the file in previous steps, thus never fails at this step; qed").to_string_lossy())),
		&serde_json::to_vec(&serde_json::to_value(a_spec).map_err(error::Generic::Serde)?)
			.map_err(error::Generic::Serde)?,
	)
}
