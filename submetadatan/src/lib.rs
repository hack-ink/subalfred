#![feature(arbitrary_enum_discriminant)]

// TODO: `module` to `pallet`

#[cfg(feature = "simple")]
pub mod simple {
	// --- std ---
	use std::convert::TryFrom;
	// --- crates.io ---
	use thiserror::Error as ThisError;
	// --- submetadatan ---
	use crate::*;

	pub type Bytes = Vec<u8>;
	pub type MetadataResult<T> = Result<T, Error>;

	#[derive(Clone, Debug, PartialEq, Eq)]
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
				.ok_or(Error::ModuleNotFound(module_name))?;
			let storages = module.storages.as_ref().ok_or(Error::StoragesNotFound)?;

			Ok(&storages.prefix)
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
				.find(|module| module.name == module_name)
				.ok_or(Error::ModuleNotFound(module_name.clone()))?;
			let storages = module.storages.as_ref().ok_or(Error::StoragesNotFound)?;
			let item = storages
				.items
				.iter()
				.find(|item| item.name == item_name)
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
				// Hint for Hasher
				r#type => Err(Error::StorageTypeMismatch {
					expected: "Map".into(),
					found: r#type.to_owned(),
				}),
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
				.position(|module| module.name == module_name)
				.ok_or(Error::ModuleNotFound(module_name.clone()))?;
			let calls = self.modules[module_index]
				.calls
				.as_ref()
				.ok_or(Error::CallsNotFound)?;
			let call_index = calls.iter().position(|call| call.name == call_name).ok_or(
				Error::CallNotFound {
					module_name,
					call_name,
				},
			)?;

			Ok([module_index as _, call_index as _])
		}
	}
	impl TryFrom<RuntimeMetadata> for Metadata {
		type Error = Error;

		fn try_from(runtime_metadata: RuntimeMetadata) -> Result<Self, Self::Error> {
			// --- submetadatan ---
			use RuntimeMetadata::*;

			match runtime_metadata {
				V12(metadata) => Ok(metadata.into()),
				_ => Err(Self::Error::NotSupportMetadataVersion(
					runtime_metadata.tag(),
				)),
			}
		}
	}
	impl From<RuntimeMetadataV12> for Metadata {
		fn from(runtime_metadata: RuntimeMetadataV12) -> Self {
			let mut metadata = Self { modules: vec![] };

			for module in runtime_metadata.modules {
				let storages = if let Some(storage) = module.storage {
					Some(Storages {
						prefix: storage.prefix,
						items: storage
							.entries
							.into_iter()
							.map(|entry| Storage {
								name: entry.name,
								r#type: entry.ty,
							})
							.collect(),
					})
				} else {
					None
				};
				let calls = if let Some(calls) = module.calls {
					Some(
						calls
							.into_iter()
							.map(|call| Call { name: call.name })
							.collect(),
					)
				} else {
					None
				};

				metadata.modules.push(Module {
					name: module.name,
					storages,
					calls,
				});
			}

			metadata
		}
	}

	#[derive(Clone, Debug, PartialEq, Eq)]
	pub struct Module {
		pub name: String,
		// pub events: Option<Vec<Event>>,
		pub storages: Option<Storages>,
		pub calls: Option<Vec<Call>>,
	}
	#[derive(Clone, Debug, PartialEq, Eq)]
	pub struct Storages {
		pub prefix: String,
		pub items: Vec<Storage>,
	}
	#[derive(Clone, Debug, PartialEq, Eq)]
	pub struct Storage {
		pub name: String,
		pub r#type: StorageEntryType,
	}
	#[derive(Clone, Debug, PartialEq, Eq)]
	pub struct Call {
		pub name: String,
	}

	#[derive(Debug, ThisError)]
	pub enum Error {
		#[error("Module `{0}` not found")]
		ModuleNotFound(String),
		#[error("This pallet doesn't have storages")]
		StoragesNotFound,
		#[error("This pallet doesn't have calls")]
		CallsNotFound,
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
		#[error("Not Support metadata version `{0}`")]
		NotSupportMetadataVersion(u8),
	}
}
#[cfg(feature = "simple")]
pub use simple::*;

#[cfg(feature = "codec")]
pub use parity_scale_codec;

// --- crates.io ---
#[cfg(feature = "codec")]
use parity_scale_codec::{Decode, Encode};
use substorager::StorageType as StorageEntryType;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct RuntimeMetadataPrefixed(pub u32, pub RuntimeMetadata);

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub enum RuntimeMetadata {
	V0 = 0,
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
	V13,
	V14,
}
impl RuntimeMetadata {
	pub fn tag(&self) -> u8 {
		unsafe { *(self as *const Self as *const u8) }
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct RuntimeMetadataV12 {
	pub modules: Vec<ModuleMetadata>,
	pub extrinsic: ExtrinsicMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct ModuleMetadata {
	pub name: String,
	pub storage: Option<StorageMetadata>,
	pub calls: Option<Vec<FunctionMetadata>>,
	pub event: Option<Vec<EventMetadata>>,
	pub constants: Vec<ModuleConstantMetadata>,
	pub errors: Vec<ErrorMetadata>,
	pub index: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct StorageMetadata {
	pub prefix: String,
	pub entries: Vec<StorageEntryMetadata>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct StorageEntryMetadata {
	pub name: String,
	pub modifier: StorageEntryModifier,
	pub ty: StorageEntryType,
	pub default: Vec<u8>,
	pub documentation: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub enum StorageEntryModifier {
	Optional,
	Default,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct FunctionMetadata {
	pub name: String,
	pub arguments: Vec<FunctionArgumentMetadata>,
	pub documentation: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct FunctionArgumentMetadata {
	pub name: String,
	pub ty: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct EventMetadata {
	pub name: String,
	pub arguments: Vec<String>,
	pub documentation: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct ModuleConstantMetadata {
	pub name: String,
	pub ty: String,
	pub value: Vec<u8>,
	pub documentation: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct ErrorMetadata {
	pub name: String,
	pub documentation: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub struct ExtrinsicMetadata {
	pub version: u8,
	pub signed_extensions: Vec<String>,
}
