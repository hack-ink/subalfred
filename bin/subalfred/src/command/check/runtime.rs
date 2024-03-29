// std
use std::{error::Error, process::Child, time::Duration};
// crates.io
use clap::{Args, ValueEnum};
// subalfred
use crate::{command::shared::Network, prelude::*};
use subalfred_core::{check::runtime, node, system};

/// Compare the runtime version of the local node with that of the live one.
#[derive(Debug, Args)]
pub(crate) struct RuntimeCmd {
	/// Path to the Node executable.
	#[arg(long, required = true, value_name = "PATH")]
	executable: String,
	/// Pass this name to the `--chain` flag to launch the local chain.
	#[arg(long, required = true, value_name = "CHAIN")]
	chain: String,
	/// HTTP RPC endpoint of the live chain.
	#[arg(long, required = true, value_name = "URI")]
	live: String,
	/// Property being targeted.
	#[arg(value_enum, long, required = true, value_name = "PROPERTY")]
	property: Property,
	#[command(flatten)]
	network: Network,
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

		let Self { executable, chain, live, property, network: Network { timeout } } = self;
		let timeout = Duration::from_secs(*timeout);
		let rpc_port = system::random_available_port()?;
		let mut node_process = node::spawn(executable, rpc_port, chain)?;
		let local = format!("http://127.0.0.1:{rpc_port}");

		match property {
			Property::Storage => {
				let result = runtime::check_storage(&local, live, timeout).await;
				let (pallet_diffs, entry_diffs) =
					map_err_and_kill_node_process(result, &mut node_process)?;

				if !pallet_diffs.is_empty() {
					pallet_diffs.into_iter().for_each(|pallet_diff| println!("{pallet_diff}"));

					println!();
				}

				entry_diffs.into_iter().for_each(|(prefix, entry_diffs)| {
					println!("Pallet {prefix}");

					entry_diffs.into_iter().for_each(|entry_diff| println!("{entry_diff}"));

					println!();
				});
			},
			Property::Version => {
				let result = runtime::check_version(&local, live, timeout).await;

				if let Some(diffs) = map_err_and_kill_node_process(result, &mut node_process)? {
					println!("{diffs}")
				}
			},
		}

		node_process.kill()?;

		Ok(())
	}
}

#[derive(Clone, Debug, ValueEnum)]
enum Property {
	Storage,
	Version,
}
