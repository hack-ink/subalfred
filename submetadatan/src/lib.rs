#[cfg(feature = "simplify-metadata")]
pub mod simplify_metadata {
	// --- std ---
	use std::convert::{TryFrom, TryInto};
	// --- crates.io ---
	use thiserror::Error as ThisError;
	// --- submetadatan ---
	use crate::*;

	pub type Bytes = Vec<u8>;
	pub type MetadataResult<T> = Result<T, Error>;

	#[derive(Clone, Debug)]
	pub struct Metadata {
		pub modules: Vec<Module>,
	}
	impl Metadata {
		pub fn storage_prefix(&self, module_name: impl Into<String>) -> MetadataResult<&str> {
			let module_name = module_name.into();
			let module = self
				.modules
				.iter()
				.find(|module| module.name == module_name)
				.ok_or(Error::ModuleNotFound { module_name })?;

			Ok(&module.storages.prefix)
		}

		pub fn storage(
			&self,
			module_name: impl Into<String>,
			item_name: impl Into<String>,
		) -> MetadataResult<&Storage> {
			let module_name = module_name.into();
			let item_name = item_name.into();
			let module = self
				.modules
				.iter()
				.find(|module| &module.name == &module_name)
				.ok_or(Error::ModuleNotFound {
					module_name: module_name.clone(),
				})?;
			let item = module
				.storages
				.items
				.iter()
				.find(|item| &item.name == &item_name)
				.ok_or(Error::StorageItemNotFound {
					module_name,
					item_name,
				})?;

			Ok(item)
		}

		pub fn storage_map_key(
			&self,
			module: impl AsRef<str>,
			item: impl AsRef<str>,
			key: impl AsRef<[u8]>,
		) -> MetadataResult<Bytes> {
			let module = module.as_ref();
			let item = item.as_ref();
			let prefix = self.storage_prefix(module)?;
			let storage = self.storage(module, item)?;

			match &storage.r#type {
				StorageEntryType::Map { hasher, .. } => {
					Ok(substorager::storage_map_key(prefix, item, (hasher, key)))
				}
				r#type => Err(Error::StorageTypeMismatch {
					expected: "Map".into(),
					found: r#type.to_owned(),
				})?,
			}
		}

		pub fn call(
			&self,
			module_name: impl Into<String>,
			call_name: impl Into<String>,
		) -> MetadataResult<[u8; 2]> {
			let module_name = module_name.into();
			let call_name = call_name.into();
			let module_index = self
				.modules
				.iter()
				.position(|module| &module.name == &module_name)
				.ok_or(Error::ModuleNotFound {
					module_name: module_name.clone(),
				})?;
			let call_index = self.modules[module_index]
				.calls
				.iter()
				.position(|call| &call.name == &call_name)
				.ok_or(Error::CallNotFound {
					module_name,
					call_name,
				})?;

			Ok([module_index as _, call_index as _])
		}
	}
	#[cfg(feature = "simplify-metadata")]
	impl TryFrom<RuntimeMetadata> for Metadata {
		type Error = Error;

		fn try_from(runtime_metadata: RuntimeMetadata) -> Result<Metadata, Self::Error> {
			// --- submetadatan ---
			use RuntimeMetadata::*;

			macro_rules! err {
				($found:expr) => {{
					Err(Error::MetadataVersionMismatch {
						expected: "V12".into(),
						found: $found.into(),
						})
					}};
			}

			match runtime_metadata {
				V0 => err!("V0"),
				V1 => err!("V1"),
				V2 => err!("V2"),
				V3 => err!("V3"),
				V4 => err!("V4"),
				V5 => err!("V5"),
				V6 => err!("V6"),
				V7 => err!("V7"),
				V8 => err!("V8"),
				V9 => err!("V9"),
				V10 => err!("V10"),
				V11 => err!("V11"),
				V12(runtime_metadata) => runtime_metadata.try_into(),
			}
		}
	}
	#[cfg(feature = "simplify-metadata")]
	impl TryFrom<RuntimeMetadataV12> for Metadata {
		type Error = Error;

		fn try_from(runtime_metadata: RuntimeMetadataV12) -> Result<Metadata, Self::Error> {
			let mut metadata = Self { modules: vec![] };

			for module in runtime_metadata.modules {
				let mut storages = Storages {
					prefix: Default::default(),
					items: vec![],
				};
				let mut calls = vec![];

				if let Some(storage) = module.storage {
					storages.prefix = storage.prefix;

					for storage in storage.entries {
						storages.items.push(Storage {
							name: storage.name,
							r#type: storage.ty,
						});
					}
				}

				if let Some(calls_) = module.calls {
					for call in calls_ {
						calls.push(Call { name: call.name });
					}
				}

				metadata.modules.push(Module {
					name: module.name,
					storages,
					calls,
				});
			}

			Ok(metadata)
		}
	}

	#[derive(Clone, Debug)]
	pub struct Module {
		pub name: String,
		// pub events: Vec<Event>,
		pub storages: Storages,
		pub calls: Vec<Call>,
	}
	#[derive(Clone, Debug)]
	pub struct Storages {
		pub prefix: String,
		pub items: Vec<Storage>,
	}
	#[derive(Clone, Debug)]
	pub struct Storage {
		pub name: String,
		pub r#type: StorageEntryType,
	}
	#[derive(Clone, Debug)]
	pub struct Call {
		pub name: String,
	}

	#[derive(Debug, ThisError)]
	pub enum Error {
		#[error("Module `{}` not found", module_name)]
		ModuleNotFound { module_name: String },
		#[error(
			"Storage item `{}` not found under module `{}`",
			module_name,
			item_name
		)]
		StorageItemNotFound {
			module_name: String,
			item_name: String,
		},
		#[error("Storage type expected `{}` but found `{:?}`", expected, found)]
		StorageTypeMismatch {
			expected: String,
			found: StorageEntryType,
		},
		#[error("Call `{}` not found under module `{}`", module_name, call_name)]
		CallNotFound {
			module_name: String,
			call_name: String,
		},
		#[error("Metadata version expected `{}` but found `{}`", expected, found)]
		MetadataVersionMismatch { expected: String, found: String },
	}
}

// --- submetadatan ---
#[cfg(feature = "simplify-metadata")]
pub use simplify_metadata::*;

// --- crates.io ---
#[cfg(feature = "codec")]
use parity_scale_codec::Decode;
use substorager::StorageType as StorageEntryType;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct RuntimeMetadataPrefixed(pub u32, pub RuntimeMetadata);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub enum RuntimeMetadata {
	V0,
	V1,
	V2,
	V3,
	V4,
	V5,
	V6,
	V7,
	V8,
	V9,
	V10,
	V11,
	V12(RuntimeMetadataV12),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct RuntimeMetadataV12 {
	pub modules: Vec<ModuleMetadata>,
	pub extrinsic: ExtrinsicMetadata,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct ModuleMetadata {
	pub name: String,
	pub storage: Option<StorageMetadata>,
	pub calls: Option<Vec<FunctionMetadata>>,
	pub event: Option<Vec<EventMetadata>>,
	pub constants: Vec<ModuleConstantMetadata>,
	pub errors: Vec<ErrorMetadata>,
	pub index: u8,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct StorageMetadata {
	pub prefix: String,
	pub entries: Vec<StorageEntryMetadata>,
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct StorageEntryMetadata {
	pub name: String,
	pub modifier: StorageEntryModifier,
	pub ty: StorageEntryType,
	pub default: Vec<u8>,
	pub documentation: Vec<String>,
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub enum StorageEntryModifier {
	Optional,
	Default,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct FunctionMetadata {
	pub name: String,
	pub arguments: Vec<FunctionArgumentMetadata>,
	pub documentation: Vec<String>,
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct FunctionArgumentMetadata {
	pub name: String,
	pub ty: String,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct EventMetadata {
	pub name: String,
	pub arguments: Vec<String>,
	pub documentation: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct ModuleConstantMetadata {
	pub name: String,
	pub ty: String,
	pub value: Vec<u8>,
	pub documentation: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct ErrorMetadata {
	pub name: String,
	pub documentation: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "codec", derive(Decode))]
pub struct ExtrinsicMetadata {
	pub version: u8,
	pub signed_extensions: Vec<String>,
}
