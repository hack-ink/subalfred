//! Substrate-like runtime checker.

// std
use std::{
	fmt::{Debug, Write},
	ops::Deref,
	time::Duration,
};
// subalfred
use crate::{node, prelude::*};
use submetadatan::{
	cmp,
	frame_metadata::{PalletStorageMetadata, StorageEntryMetadata},
	scale_info::{form::PortableForm, PortableRegistry},
	LatestRuntimeMetadata,
};

/// Retrieve the runtime versions of two nodes by using their RPC endpoints, compare the versions,
/// and present the differences in markdown diff format.
pub async fn check_version(a_uri: &str, b_uri: &str, timeout: Duration) -> Result<Option<String>> {
	const E_WRITE_TO_STRING_NEVER_FAILS: &str = "[core::check] write to string never fails; qed";

	let (a, b) = {
		let (a, b) = futures::join!(
			node::runtime_version(a_uri, None::<()>, timeout),
			node::runtime_version(b_uri, None::<()>, timeout)
		);

		(a?, b?)
	};

	if a == b {
		return Ok(None);
	}
	let mut result = String::new();

	for (a, b) in format!("{a:#?}").lines().zip(format!("{b:#?}").lines()) {
		if a == b {
			writeln!(result, "{a}").expect(E_WRITE_TO_STRING_NEVER_FAILS);
		} else {
			// Skip the first space to suit the format.
			writeln!(result, "-{}", &b[1..]).expect(E_WRITE_TO_STRING_NEVER_FAILS);
			writeln!(result, "+{}", &a[1..]).expect(E_WRITE_TO_STRING_NEVER_FAILS);
		}
	}

	Ok(Some(result))
}

/// Fetch the runtime storage of two nodes using their RPC endpoints. compare the storage and return
/// the differences in markdown diff format.
pub async fn check_storage(
	a_uri: &str,
	b_uri: &str,
	timeout: Duration,
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

	let mut pallet_diff = Vec::new();
	let mut entry_diffs = Vec::new();
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

		let (a, b) = futures::join!(
			node::runtime_metadata(a_uri, None::<()>, timeout),
			node::runtime_metadata(b_uri, None::<()>, timeout)
		);
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
			pallet_diff.push(a.output());

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
						entry_diffs.iter().position(|(p, _): &(String, Vec<String>)| p == prefix)
					{
						entry_diffs[i].1.push(a.output());
					} else {
						entry_diffs.push((prefix.to_string(), vec![a.output()]));
					}

					break;
				};

				if a.opposite(b) && cmp::storage_entry(&a_types, &b_types, a, b) {
					j += 2;
				} else {
					// TODO: get type diffs
					if let Some(i) = entry_diffs.iter().position(|(p, _)| p == prefix) {
						entry_diffs[i].1.push(a.output());
					} else {
						entry_diffs.push((prefix.to_string(), vec![a.output()]));
					}

					j += 1;
				}
			}

			i += 2;
		} else {
			pallet_diff.push(a.output());

			i += 1;
		}
	}

	Ok((pallet_diff, entry_diffs))
}
