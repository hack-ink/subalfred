// --- crates.io ---
use isahc::AsyncReadResponseExt;
use parity_scale_codec::Decode;
use submetadatan::{RuntimeMetadata, RuntimeMetadataPrefixed, RuntimeMetadataV12};
use tracing::trace;
// --- subalfred ---
use crate::{substrate::node::RpcResult, AnyResult, Subalfred};

impl Subalfred {
	pub async fn runtime_metadata(uri: impl AsRef<str>) -> AnyResult<RuntimeMetadataV12> {
		let result = {
			let mut v = vec![];
			subrpcer::send_rpc(uri, subrpcer::state::get_metadata())
				.await?
				.copy_to(&mut v)
				.await?;

			serde_json::from_slice::<RpcResult>(&v)?.result
		};
		let raw_runtime_metadata_prefixed =
			array_bytes::hex2bytes(result.as_str().unwrap()).unwrap();
		let runtime_metadata_prefixed =
			RuntimeMetadataPrefixed::decode(&mut &*raw_runtime_metadata_prefixed)?;
		let runtime_metadata =
			if let RuntimeMetadata::V12(runtime_metadata) = runtime_metadata_prefixed.1 {
				runtime_metadata
			} else {
				unimplemented!()
			};

		trace!("{:#?}", runtime_metadata);

		Ok(runtime_metadata)
	}

	pub async fn list_module(uri: impl AsRef<str>) -> AnyResult<Vec<String>> {
		Ok(Self::runtime_metadata(uri)
			.await?
			.modules
			.into_iter()
			.map(|module| format!("{}: {}", module.name, module.index))
			.collect())
	}

	pub async fn list_storage_keys(uri: impl AsRef<str>) -> AnyResult<Vec<String>> {
		Ok(Self::runtime_metadata(uri)
			.await?
			.modules
			.into_iter()
			.filter_map(|module| {
				module.storage.map(|storage| {
					storage
						.entries
						.iter()
						.map(|entry| {
							format!(
								"{}{}: {}",
								storage.prefix,
								entry.name,
								storage_key(storage.prefix.as_bytes(), entry.name.as_bytes())
							)
						})
						.collect::<Vec<_>>()
				})
			})
			.flatten()
			.collect())
	}
}

fn storage_key(prefix: impl AsRef<[u8]>, item: impl AsRef<[u8]>) -> String {
	let mut storage_key = String::from("0x");

	storage_key.push_str(&array_bytes::bytes2hex("", &subhasher::twox_128(prefix)));
	storage_key.push_str(&array_bytes::bytes2hex("", &subhasher::twox_128(item)));

	storage_key
}
