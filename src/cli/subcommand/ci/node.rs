// --- std ---
use std::{
	convert::TryFrom,
	fmt::Debug,
	io::{BufRead, BufReader},
	mem,
	path::PathBuf,
	process::{Child, Command, Stdio},
};
// --- crates.io ---
use isahc::ReadResponseExt;
use parity_scale_codec::Decode;
use serde::Deserialize;
use serde_json::Value;
use structopt::{clap::arg_enum, StructOpt};
use submetadatan::{Metadata, RuntimeMetadataPrefixed, Storages};
use subrpcer::client::i;
// --- subalfred ---
use crate::AnyResult;

const LOCAL_NODE_RPC_END_POINT: &str = "http://localhost:9933";

// TODO: custom network
arg_enum! {
	#[derive(Debug)]
	pub enum Chain {
		Darwinia,
		Crab,
		Pangolin,
		Pangoro,
	}
}
impl Chain {
	fn rpc_endpoint(&self) -> &str {
		match self {
			Chain::Darwinia => "https://rpc.darwinia.network",
			Chain::Crab => "https://crab-rpc.darwinia.network",
			Chain::Pangolin => "https://pangolin-rpc.darwinia.network",
			Chain::Pangoro => "https://pangoro-rpc.darwinia.network",
		}
	}
}

#[derive(Clone, Debug)]
enum ChainType<T> {
	Local(T),
	Live(T),
}
impl<T> ChainType<T> {
	fn wrap_local(t: T) -> Self {
		Self::Local(t)
	}

	fn wrap_live(t: T) -> Self {
		Self::Live(t)
	}

	fn inner(&self) -> &T {
		match self {
			ChainType::Local(t) => t,
			ChainType::Live(t) => t,
		}
	}

	fn is_live(&self) -> bool {
		match self {
			ChainType::Live(_) => true,
			_ => false,
		}
	}

	fn output<'a, 'b, F, D>(&'a self, prefix: &str, f: F) -> String
	where
		'a: 'b,
		F: FnOnce(&'b T) -> D,
		D: Debug,
	{
		if self.is_live() {
			format!("{}", format!("- {}: {:?}", prefix, f(self.inner())))
		} else {
			format!("{}", format!("+ {}: {:?}", prefix, f(self.inner())))
		}
	}
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
	pub spec_name: String,
	pub impl_name: String,
	pub authoring_version: u32,
	pub spec_version: u32,
	pub impl_version: u32,
	pub transaction_version: u32,
}

#[derive(Debug, StructOpt)]
pub struct NodeCmd {
	#[structopt(short, long, required = true, takes_value = true)]
	executable: PathBuf,
	#[structopt(
		help = "Specific chain (non case sensitive)",
		short,
		long,
		case_insensitive = true,
		required = true,
		takes_value = true,
		possible_values = &Chain::variants(),
		value_name = "CHAIN"
	)]
	chain: Chain,
}
impl NodeCmd {
	fn spawn_local_node(&self) -> AnyResult<Child> {
		let mut local_node = Command::new(&self.executable)
			.stdout(Stdio::null())
			.stderr(Stdio::piped())
			.args(&["--chain", &format!("{}-dev", self.chain), "--tmp"])
			.spawn()?;
		let output = BufReader::new(local_node.stderr.take().ok_or(anyhow::anyhow!(""))?);

		for line in output.lines().filter_map(Result::ok) {
			if line.contains("Idle") {
				break;
			}
		}

		Ok(local_node)
	}

	fn fetch_runtime_version(uri: impl AsRef<str>) -> AnyResult<RuntimeVersion> {
		let result = i::send_rpc(uri, subrpcer::state::get_runtime_version())?
			.json::<Value>()?
			.get_mut("result")
			.ok_or(anyhow::anyhow!(""))?
			.take();
		let runtime_version = serde_json::from_value(result)?;

		Ok(runtime_version)
	}

	fn fetch_metadata(uri: impl AsRef<str>) -> AnyResult<Vec<Storages>> {
		let metadata = {
			let mut response =
				i::send_rpc(uri, subrpcer::state::get_metadata())?.json::<Value>()?;
			let hex_codec_metadata = response
				.get_mut("result")
				.map(|v| v.take())
				.ok_or(anyhow::anyhow!(""))?
				.as_str()
				.map(ToOwned::to_owned)
				.ok_or(anyhow::anyhow!(""))?;
			let codec_metadata =
				array_bytes::hex2bytes(hex_codec_metadata).map_err(|_| anyhow::anyhow!(""))?;
			let metadata_prefixed = RuntimeMetadataPrefixed::decode(&mut &*codec_metadata)?;
			let metadata = Metadata::try_from(metadata_prefixed.1)?;

			metadata
		};
		let storages = metadata
			.modules
			.into_iter()
			.filter_map(|module| module.storages)
			.collect();

		Ok(storages)
	}

	fn merge(mut storages: Vec<Storages>) -> Vec<Storages> {
		let len = storages.len();

		if len < 2 {
			return storages;
		}

		storages.sort_by(|a, b| a.prefix.as_str().cmp(b.prefix.as_str()));

		let mut i = 0;
		let mut j = 1;

		while i != len - 1 {
			let a = storages[i].prefix.clone();

			while j != len {
				if &a == &storages[j].prefix {
					let mut items = mem::replace(&mut storages[j].items, vec![]);

					storages[i].items.append(&mut items);

					j += 1;
				} else {
					i = j;
					j += 1;

					break;
				}
			}
		}

		storages
			.into_iter()
			.filter(|storages| !storages.items.is_empty())
			.collect()
	}

	fn wrap<'a, T: 'a, F>(values: &'a [T], f: F) -> Vec<ChainType<&'a T>>
	where
		F: Fn(&'a T) -> ChainType<&'a T>,
	{
		values.iter().map(f).collect()
	}

	pub fn run_runtime_version_check(&self) -> AnyResult<()> {
		let mut local_node = self.spawn_local_node()?;
		let local_runtime_version = Self::fetch_runtime_version(LOCAL_NODE_RPC_END_POINT)?;
		let chain_runtime_version = Self::fetch_runtime_version(self.chain.rpc_endpoint())?;

		local_node.kill()?;

		if local_runtime_version == chain_runtime_version {
			return Ok(());
		}

		let mut runtime_version = "Runtime Version {".to_string();

		macro_rules! colored_diff {
			($($field:ident),*) => {
				$(
					if local_runtime_version.$field != chain_runtime_version.$field {
						runtime_version.push_str(&format!("\n-\t{}: {}", stringify!($field), &chain_runtime_version.$field));
						runtime_version.push_str(&format!("\n+\t{}: {}", stringify!($field), &local_runtime_version.$field));
					} else {
						runtime_version.push_str(&format!("\n\t{}: {}", stringify!($field), &local_runtime_version.$field));
					}
				)*
			};
		}

		colored_diff![
			spec_name,
			impl_name,
			authoring_version,
			spec_version,
			impl_version,
			transaction_version
		];

		runtime_version.push_str("\n}");

		println!("{}", runtime_version);

		Ok(())
	}

	pub fn run_storage_prefix_check(&self) -> AnyResult<()> {
		let mut local_node = self.spawn_local_node()?;
		let local_storages = Self::merge(Self::fetch_metadata(LOCAL_NODE_RPC_END_POINT)?);
		let live_storages = Self::merge(Self::fetch_metadata(self.chain.rpc_endpoint())?);
		let mut storages = [
			Self::wrap(&local_storages, ChainType::wrap_local),
			Self::wrap(&live_storages, ChainType::wrap_live),
		]
		.concat();

		storages.sort_by(|a, b| a.inner().prefix.as_str().cmp(b.inner().prefix.as_str()));

		let mut i = 0;

		while i != storages.len() {
			let a = &storages[i];
			let b = if let Some(b) = storages.get(i + 1) {
				b
			} else {
				println!("{}", a.output("Pallet", |s| &s.prefix));

				break;
			};

			if a.inner().prefix == b.inner().prefix {
				let mut items = [
					Self::wrap(&a.inner().items, ChainType::wrap_local),
					Self::wrap(&b.inner().items, ChainType::wrap_live),
				]
				.concat();

				items.sort_by(|a, b| a.inner().name.as_str().cmp(b.inner().name.as_str()));

				let mut j = 0;
				let mut outputs = vec![];

				while j != items.len() {
					let a = &items[j];
					let b = if let Some(b) = items.get(j + 1) {
						b
					} else {
						outputs.push(a.output("\tItem", |s| s));

						break;
					};

					if a.inner() == b.inner() {
						j += 2;
					} else {
						outputs.push(a.output("\tItem", |s| s));

						j += 1;
					}
				}

				if !outputs.is_empty() {
					println!("Pallet {}:", a.inner().prefix);

					for output in outputs {
						println!("{}", output);
					}

					println!();
				}

				i += 2;
			} else {
				println!("{}\n", a.output("Pallet", |s| &s.prefix));

				i += 1;
			}
		}

		local_node.kill()?;

		Ok(())
	}
}
