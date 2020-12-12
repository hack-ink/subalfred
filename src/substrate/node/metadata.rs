// --- crates.io ---
use isahc::ResponseExt;
use parity_scale_codec::Decode;
use submetadatan::{RuntimeMetadata, RuntimeMetadataPrefixed, RuntimeMetadataV12};
use tracing::trace;
// --- subalfred ---
use crate::{substrate::node::RpcResult, AnyResult, Subalfred};

impl Subalfred {
	pub async fn runtime_metadata(uri: impl AsRef<str>) -> AnyResult<RuntimeMetadataV12> {
		let result = subrpcer::send_rpc(uri, subrpcer::state::get_metadata())
			.await?
			.json::<RpcResult>()?
			.result;
		let raw_runtime_metadata_prefixed = array_bytes::bytes(result.as_str().unwrap()).unwrap();
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
			.map(|module| module.name)
			.collect())
	}
}
