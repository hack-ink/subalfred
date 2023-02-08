//! Subalfred core runtime state library.

#[cfg(test)] mod test;

pub mod diff;
pub use diff::*;

pub mod export;
pub use export::*;

pub mod fork_off;
pub use fork_off::*;

pub mod insert;
pub use insert::*;

pub mod r#override;
pub use r#override::*;

// std
use std::{path::Path, thread};
// subalfred
use crate::{prelude::*, system};
use subspector::ChainSpec;

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
