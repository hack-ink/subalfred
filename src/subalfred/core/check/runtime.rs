// std
use std::{
	fmt::{Debug, Write},
	ops::Deref,
};
// hack-ink
use crate::core::{error, node, Result};
use submetadatan::{
	form::PortableForm, LatestRuntimeMetadata, PalletStorageMetadata, PortableRegistry,
	StorageEntryMetadata,
};

const E_WRITE_TO_STRING_FAILED: &str = "[core::check] write to a `String` will never failed";

/// Compare the nodes' runtime versions.
/// If there is no difference, return `None`.
pub async fn check_version(a_uri: &str, b_uri: &str) -> Result<Option<String>> {
	let (a, b) = {
		let (a, b) = futures::join!(node::runtime_version(a_uri), node::runtime_version(b_uri));

		(a?, b?)
	};

	if a == b {
		return Ok(None);
	}
	let mut result = String::new();
	let to_e = |_| error::Generic::AlmostImpossible(E_WRITE_TO_STRING_FAILED);

	for (a, b) in format!("{a:#?}").lines().zip(format!("{b:#?}").lines()) {
		if a == b {
			writeln!(result, "{a}").map_err(to_e)?;
		} else {
			// Skip the first space to suit the format.
			writeln!(result, "-{}", &b[1..]).map_err(to_e)?;
			writeln!(result, "+{}", &a[1..]).map_err(to_e)?;
		}
	}

	Ok(Some(result))
}

/// Compare the nodes' runtime storages.
pub async fn check_storage(
	a_uri: &str,
	b_uri: &str,
) -> Result<(Vec<String>, Vec<(String, Vec<String>)>)> {
	trait Output {
		type D: Debug;

		fn ty(&self) -> &str;
		fn detail(&self) -> Self::D;
	}
	impl<'a> Output for &'a PalletStorageMetadata<PortableForm> {
		type D = &'a str;

		fn ty(&self) -> &str {
			"Pallet"
		}

		fn detail(&self) -> Self::D {
			&self.prefix
		}
	}
	impl Output for &StorageEntryMetadata<PortableForm> {
		type D = Self;

		fn ty(&self) -> &str {
			"Entry"
		}

		fn detail(&self) -> Self::D {
			self
		}
	}

	enum Either<T> {
		A(T),
		B(T),
	}
	impl<T> Either<T> {
		fn a(v: T) -> Self {
			Either::A(v)
		}

		fn b(v: T) -> Self {
			Either::B(v)
		}

		fn opposite(&self, other: &Self) -> bool {
			!matches!((self, other), (Either::A(_), Either::A(_)) | (Either::B(_), Either::B(_)))
		}

		fn output(&self) -> String
		where
			T: Output,
		{
			match self {
				Either::A(v) => format!("+ {}: {:?}", v.ty(), v.detail()),
				Either::B(v) => format!("- {}: {:?}", v.ty(), v.detail()),
			}
		}
	}
	impl<T> Deref for Either<T> {
		type Target = T;

		fn deref(&self) -> &Self::Target {
			match self {
				Either::A(a) => a,
				Either::B(b) => b,
			}
		}
	}

	let mut pallets_diff = Vec::new();
	let mut entries_diffs = Vec::new();
	let ((a_types, a_storages), (b_types, b_storages)) = {
		// TODO: consider moving this to util
		fn parse_metadata(
			metadata: LatestRuntimeMetadata,
		) -> (PortableRegistry, Vec<PalletStorageMetadata<PortableForm>>) {
			(
				metadata.types,
				// Skip the pallet which doesn't have a storage.
				metadata.pallets.into_iter().filter_map(|pallet| pallet.storage).collect(),
			)
		}

		let (a, b) = futures::join!(node::runtime_metadata(a_uri), node::runtime_metadata(b_uri));
		let (a, b) = (a?, b?);

		(parse_metadata(a), parse_metadata(b))
	};
	let mut storages = a_storages
		.iter()
		.map(Either::a)
		.chain(b_storages.iter().map(Either::b))
		.collect::<Vec<_>>();

	storages.sort_by(|a, b| a.prefix.cmp(&b.prefix));

	// Avoid mutable.
	let storages = storages;
	let mut i = 0;

	while i != storages.len() {
		let a = &storages[i];
		let b = if let Some(b) = storages.get(i + 1) {
			b
		} else {
			pallets_diff.push(a.output());

			break;
		};

		if a.prefix == b.prefix {
			let prefix = &a.prefix;
			let mut entries = a
				.entries
				.iter()
				.map(Either::a)
				.chain(b.entries.iter().map(Either::b))
				.collect::<Vec<_>>();

			entries.sort_by(|a, b| a.name.cmp(&b.name));

			// Avoid mutable.
			let entries = entries;
			let mut j = 0;

			while j != entries.len() {
				let a = &entries[j];
				let b = if let Some(b) = entries.get(j + 1) {
					b
				} else {
					if let Some(i) =
						entries_diffs.iter().position(|(p, _): &(String, Vec<String>)| p == prefix)
					{
						entries_diffs[i].1.push(a.output());
					} else {
						entries_diffs.push((prefix.to_string(), vec![a.output()]));
					}

					break;
				};

				if a.opposite(b) && submetadatan::cmp_storage_entry(&a_types, &b_types, a, b) {
					j += 2;
				} else {
					// TODO: get type diffs
					if let Some(i) = entries_diffs.iter().position(|(p, _)| p == prefix) {
						entries_diffs[i].1.push(a.output());
					} else {
						entries_diffs.push((prefix.to_string(), vec![a.output()]));
					}

					j += 1;
				}
			}

			i += 2;
		} else {
			pallets_diff.push(a.output());

			i += 1;
		}
	}

	Ok((pallets_diff, entries_diffs))
}
