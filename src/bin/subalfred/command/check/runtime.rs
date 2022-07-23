// std
use std::{error::Error, process::Child};
// crates.io
use clap::{ArgEnum, Args};
// hack-ink
use crate::prelude::*;
use subalfred::core::{check::runtime, node, system};

/// Compare the local node runtime version with live's.
#[derive(Debug, Args)]
pub(crate) struct RuntimeCmd {
	/// Path to the executable.
	#[clap(long, required = true, value_name = "PATH")]
	executable: String,
	/// Chain name.
	#[clap(long, required = true, value_name = "NAME")]
	chain: String,
	/// Live chain's RPC HTTP endpoint.
	#[clap(long, required = true, value_name = "URI")]
	live: String,
	/// The property to check.
	#[clap(arg_enum, long, required = true, value_name = "PROPERTY")]
	property: Property,
}
impl RuntimeCmd {
	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		fn map_err_and_kill_node_process<T, E>(
			result: Result<T, E>,
			node_process: &mut Child,
		) -> Result<T>
		where
			E: 'static + Error + Send + Sync,
		{
			if result.is_err() {
				node_process.kill()?;
			}

			Ok(result?)
		}

		let Self { executable, chain, live, property } = self;
		let rpc_port = system::random_available_port()?;
		let mut node_process = node::spawn(executable, rpc_port, chain)?;
		let local = format!("http://127.0.0.1:{rpc_port}");

		match property {
			Property::Storage => {
				let result = runtime::check_storage(&local, live).await;
				let (pallet_diffs, entry_diffs) =
					map_err_and_kill_node_process(result, &mut node_process)?;

				if !pallet_diffs.is_empty() {
					pallet_diffs.into_iter().for_each(|pallet_diff| println!("{pallet_diff}"));

					println!();
				}

				entry_diffs.into_iter().for_each(|(prefix, entry_diffs)| {
					println!("Pallet {prefix}",);

					entry_diffs.into_iter().for_each(|entry_diff| println!("{entry_diff}"));

					println!();
				});
			},
			Property::Version => {
				let result = runtime::check_version(&local, live).await;

				if let Some(diffs) = map_err_and_kill_node_process(result, &mut node_process)? {
					println!("{diffs}")
				}
			},
		}

		node_process.kill()?;

		Ok(())
	}
}

/// Runtime's property.
#[derive(Clone, Debug, ArgEnum)]
enum Property {
	/// Check the runtime storage.
	Storage,
	/// Check the runtime version.
	Version,
}
