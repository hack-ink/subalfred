// std
use std::mem;
// hack-ink
use crate::{node::ChainType, *};
use submetadatan::{
	form::PortableForm, PalletStorageMetadata, PortableRegistry, StorageEntryMetadata,
};

impl Executor {
	const LOCAL_NODE_RPC_END_POINT: &'static str = "http://localhost:9933";

	pub fn check_runtime_version(
		executable: &str,
		chain: &str,
		live_rpc_endpoint: &str,
	) -> AnyResult<()> {
		let mut local_node = Self::spawn_local_node(executable, chain)?;
		let local_runtime_version = Self::fetch_runtime_version(Self::LOCAL_NODE_RPC_END_POINT)?;
		let live_runtime_version = Self::fetch_runtime_version(live_rpc_endpoint)?;

		local_node.kill()?;

		if local_runtime_version == live_runtime_version {
			return Ok(());
		}

		let mut runtime_version = "Runtime Version {".to_owned();

		macro_rules! colored_diff {
				($($field:ident),*) => {
					$(
						if local_runtime_version.$field != live_runtime_version.$field {
							runtime_version.push_str(&format!("\n-\t{}: {}", stringify!($field), &live_runtime_version.$field));
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

	pub fn check_storage_prefix(
		executable: &str,
		chain: &str,
		live_rpc_endpoint: &str,
	) -> AnyResult<()> {
		let mut local_node = Self::spawn_local_node(executable, chain)?;
		let (local_types, local_storages) = fetch_storage_metadata(Self::LOCAL_NODE_RPC_END_POINT)?;
		let (live_types, live_storages) = fetch_storage_metadata(live_rpc_endpoint)?;
		let mut storages = [
			ChainType::wrap(&local_storages, ChainType::local),
			ChainType::wrap(&live_storages, ChainType::live),
		]
		.concat();

		// There can't be two same name in the `construct_runtime!`.
		// After this sorting, we got:
		// 	[local_a, live_a, local_b...], both of the chain have pallet a
		// or
		// 	[local_a, local_b, live_b...], live chain doesn't have pallet a
		storages.sort_by(|a, b| a.inner().prefix.cmp(&b.inner().prefix));

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
				let mut entries = [
					ChainType::wrap(&a.inner().entries, ChainType::local),
					ChainType::wrap(&b.inner().entries, ChainType::live),
				]
				.concat();

				// Same idea as above.
				entries.sort_by(|a, b| a.inner().name.cmp(&b.inner().name));

				let mut j = 0;
				let mut outputs = vec![];

				while j != entries.len() {
					let a = &entries[j];
					let b = if let Some(b) = entries.get(j + 1) {
						b
					} else {
						outputs.push(a.output("\tEntry", |s| s));

						break;
					};

					if cmp_wrapped_storage_entry(&local_types, &live_types, a, b) {
						j += 2;
					} else {
						outputs.push(a.output("\tEntry", |s| s));

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

fn fetch_storage_metadata(
	uri: &str,
) -> AnyResult<(PortableRegistry, Vec<PalletStorageMetadata<PortableForm>>)> {
	let metadata = Executor::fetch_metadata(uri)?;
	let types = metadata.types;
	let storages = metadata
		.pallets
		.into_iter()
		.filter_map(|pallet| pallet.storage)
		.collect();
	let storages = merge_storages(storages);

	Ok((types, storages))
}

// TODO: Remove this after all pallets switch to FRAME V2.
fn merge_storages(
	mut storages: Vec<PalletStorageMetadata<PortableForm>>,
) -> Vec<PalletStorageMetadata<PortableForm>> {
	let len = storages.len();

	if len < 2 {
		return storages;
	}

	storages.sort_by(|a, b| a.prefix.cmp(&b.prefix));

	let mut i = 0;
	let mut j = 1;

	while i != len - 1 {
		let a = storages[i].prefix.clone();

		while j != len {
			if &a == &storages[j].prefix {
				let mut entries = mem::replace(&mut storages[j].entries, vec![]);

				storages[i].entries.append(&mut entries);

				j += 1;
			} else {
				i = j;
				j += 1;

				break;
			}
		}
	}

	let storages = storages
		.into_iter()
		.filter(|storage| !storage.entries.is_empty())
		.collect();

	storages
}

fn cmp_wrapped_storage_entry(
	local_types: &PortableRegistry,
	live_types: &PortableRegistry,
	a: &ChainType<&StorageEntryMetadata<PortableForm>>,
	b: &ChainType<&StorageEntryMetadata<PortableForm>>,
) -> bool {
	a.opposite_to(b)
		// The sort is stable (i.e., does not reorder equal elements).
		// So, `a` must be the `ChainType::Local` and `b` must be the `ChainType::Live` (Because of the `concat![local, live]`).
		&& submetadatan::cmp_storage_entry(local_types, live_types, a.inner(), b.inner())
}
