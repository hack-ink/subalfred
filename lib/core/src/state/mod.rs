//! State core library.

pub mod diff;
pub use diff::*;

pub mod export;
pub use export::*;

pub mod fork_off;
pub use fork_off::*;

pub mod r#override;
pub use r#override::*;

// std
use std::{
	path::{Path, PathBuf},
	thread,
};
// crates.io
#[cfg(feature = "clap")] use clap::Args;
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// hack-ink
use crate::{prelude::*, system};

/// Two state configurations.
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug)]
pub struct TwoStateConfig {
	/// The path to the state a.
	#[cfg_attr(feature = "clap", arg(required = true, value_name = "PATH"))]
	pub a: PathBuf,
	/// The path to the second state b.
	#[cfg_attr(feature = "clap", arg(required = true, value_name = "PATH"))]
	pub b: PathBuf,
}

// TODO: doc & move this to substrate-minimal
#[allow(missing_docs)]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainSpec {
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub id: String,
	#[serde(default)]
	pub chain_type: String,
	#[serde(default)]
	pub boot_nodes: Vec<String>,
	pub telemetry_endpoints: Option<Value>,
	pub protocol_id: Option<String>,
	// TODO: for latest substrate version
	// #[serde(default = "Default::default", skip_serializing_if = "Option::is_none")]
	// pub fork_id: Option<String>,
	pub properties: Option<Value>,
	#[serde(default, flatten)]
	pub extensions: Value,
	#[serde(default)]
	pub consensus_engine: (),
	#[serde(default)]
	pub genesis: Genesis,
	#[serde(default)]
	pub code_substitutes: Value,
}
// TODO: doc & move this to substrate-minimal
#[allow(missing_docs)]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genesis {
	#[serde(default)]
	pub raw: Raw,
}
// TODO: doc & move this to substrate-minimal
#[allow(missing_docs)]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Raw {
	#[serde(default)]
	pub top: FxHashMap<String, String>,
	#[serde(default)]
	pub children_default: Value,
}

fn read_chain_spec_concurrent<P, P_>(a: P, b: P_) -> Result<(ChainSpec, ChainSpec)>
where
	P: Send + AsRef<Path>,
	P_: Send + AsRef<Path>,
{
	let (a, b) = thread::scope(|scope| {
		let a = scope.spawn(|| system::read_file_to_struct::<_, ChainSpec>(a));
		let b = scope.spawn(|| system::read_file_to_struct::<_, ChainSpec>(b));

		(a.join(), b.join())
	});

	Ok((a.map_err(error::quick_err)??, b.map_err(error::quick_err)??))
}

fn override_top(mut a: ChainSpec, b: ChainSpec) -> ChainSpec {
	let a_state = &mut a.genesis.raw.top;
	let b_state = b.genesis.raw.top;

	b_state.into_iter().for_each(|(k, v)| {
		a_state.insert(k, v);
	});

	a
}

fn write_to_custom_extension_file(
	base_path: &Path,
	file_extension: &str,
	chain_spec: ChainSpec,
) -> Result<()> {
	system::write_data_to_file(
		base_path.with_file_name(format!(
			"{}.{}",
			base_path.file_name().expect("[core::state] able to read the file in previous steps, thus never fails at this step; qed").to_string_lossy(),
			file_extension
		)),
		&serde_json::to_vec(&serde_json::to_value(chain_spec).map_err(error::Generic::Serde)?)
			.map_err(error::Generic::Serde)?,
	)
}
