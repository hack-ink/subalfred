pub use frame_metadata::{RuntimeMetadataV14 as LatestRuntimeMetadata, *};
pub use scale_info::*;

// std
use std::any::TypeId;
// crates.io
use scale_info::{
	form::PortableForm, interner::UntrackedSymbol, Field, Type, TypeDef, TypeParameter, Variant,
};
use thiserror::Error as ThisError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
	// #[error("Module `{0}` not found")]
	// ModuleNotFound(String),
	// #[error("This pallet doesn't have storages")]
	// StoragesNotFound,
	// #[error("This pallet doesn't have calls")]
	// CallsNotFound,
	// #[error(
	// 	"Storage item `{}` not found under module `{}`",
	// 	module_name,
	// 	item_name
	// )]
	// StorageItemNotFound {
	// 	module_name: String,
	// 	item_name: String,
	// },
	// #[error("Storage type expected `{}` but found `{:?}`", expected, found)]
	// StorageTypeMismatch {
	// 	expected: String,
	// 	found: StorageEntryType,
	// },
	// #[error("Call `{}` not found under module `{}`", module_name, call_name)]
	// CallNotFound {
	// 	module_name: String,
	// 	call_name: String,
	// },
	#[error("Not Support metadata version `{0}`")]
	NotSupportMetadataVersion(u32),
}

pub fn metadata(metadata: RuntimeMetadataPrefixed) -> Result<LatestRuntimeMetadata> {
	match metadata.1 {
		RuntimeMetadata::V14(metadata) => Ok(metadata),
		metadata => Err(Error::NotSupportMetadataVersion(metadata.version())),
	}
}

pub fn cmp_storage_entry(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &StorageEntryMetadata<PortableForm>,
	b: &StorageEntryMetadata<PortableForm>,
) -> bool {
	a.name == b.name
		&& a.modifier == b.modifier
		&& a.default == b.default
		&& a.docs == b.docs
		&& cmp_storage_entry_type(a_types, b_types, &a.ty, &b.ty)
}

pub fn cmp_storage_entry_type(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &StorageEntryType<PortableForm>,
	b: &StorageEntryType<PortableForm>,
) -> bool {
	match a {
		StorageEntryType::Plain(a) => match b {
			StorageEntryType::Plain(b) => cmp_untracked_symbol(a_types, b_types, a, b),
			_ => false,
		},
		StorageEntryType::Map {
			hashers: a_hashers,
			key: a_key,
			value: a_value,
		} => match b {
			StorageEntryType::Map {
				hashers: b_hashers,
				key: b_key,
				value: b_value,
			} => {
				a_hashers == b_hashers
					&& cmp_untracked_symbol(a_types, b_types, a_key, b_key)
					&& cmp_untracked_symbol(a_types, b_types, a_value, b_value)
			}
			_ => false,
		},
	}
}

pub fn cmp_untracked_symbol(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &UntrackedSymbol<TypeId>,
	b: &UntrackedSymbol<TypeId>,
) -> bool {
	cmp_type(
		a_types,
		b_types,
		a_types.resolve(a.id()),
		b_types.resolve(b.id()),
	)
}

pub fn cmp_type(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: Option<&Type<PortableForm>>,
	b: Option<&Type<PortableForm>>,
) -> bool {
	if let Some(a) = a {
		if let Some(b) = b {
			a.path() == b.path()
				&& a.docs() == b.docs()
				&& cmp_type_params(a_types, b_types, a.type_params(), b.type_params())
				&& cmp_type_def(a_types, b_types, a.type_def(), b.type_def())
		} else {
			false
		}
	} else {
		if b.is_none() {
			true
		} else {
			false
		}
	}
}

pub fn cmp_type_params(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &[TypeParameter<PortableForm>],
	b: &[TypeParameter<PortableForm>],
) -> bool {
	if a.is_empty() && b.is_empty() {
		return true;
	}
	if a.len() != b.len() {
		return false;
	}

	for (a, b) in a.iter().zip(b.iter()) {
		if a.name() != b.name() {
			return false;
		}

		if let Some(a_type) = a.ty() {
			if let Some(b_type) = b.ty() {
				return cmp_untracked_symbol(a_types, b_types, a_type, b_type);
			} else {
				return false;
			}
		} else {
			if !b.ty().is_none() {
				return false;
			}
		}
	}

	true
}

pub fn cmp_type_def(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &TypeDef<PortableForm>,
	b: &TypeDef<PortableForm>,
) -> bool {
	match a {
		TypeDef::Composite(a) => match b {
			TypeDef::Composite(b) => cmp_fields(a_types, b_types, a.fields(), b.fields()),
			_ => false,
		},
		TypeDef::Variant(_a) => match b {
			TypeDef::Variant(_b) => {
				// TODO: check variants
				// cmp_variants(a_types, b_types, a.variants(), b.variants())
				true
			}
			_ => false,
		},
		TypeDef::Sequence(a) => match b {
			TypeDef::Sequence(b) => {
				cmp_untracked_symbol(a_types, b_types, a.type_param(), b.type_param())
			}
			_ => false,
		},
		TypeDef::Array(a) => match b {
			TypeDef::Array(b) => {
				a.len() == b.len()
					&& cmp_untracked_symbol(a_types, b_types, a.type_param(), b.type_param())
			}
			_ => false,
		},
		TypeDef::Tuple(a) => match b {
			TypeDef::Tuple(b) => {
				let a = a.fields();
				let b = b.fields();

				if a.is_empty() && b.is_empty() {
					return true;
				}
				if a.len() != b.len() {
					return false;
				}

				for (a, b) in a.iter().zip(b.iter()) {
					if !cmp_untracked_symbol(a_types, b_types, a, b) {
						return false;
					}
				}

				true
			}
			_ => false,
		},
		TypeDef::Primitive(a) => match b {
			TypeDef::Primitive(b) => a == b,
			_ => false,
		},
		TypeDef::Compact(a) => match b {
			TypeDef::Compact(b) => {
				cmp_untracked_symbol(a_types, b_types, a.type_param(), b.type_param())
			}
			_ => false,
		},
		TypeDef::BitSequence(a) => match b {
			TypeDef::BitSequence(b) => {
				cmp_untracked_symbol(a_types, b_types, a.bit_order_type(), b.bit_order_type())
					&& cmp_untracked_symbol(
						a_types,
						b_types,
						a.bit_store_type(),
						b.bit_store_type(),
					)
			}

			_ => false,
		},
	}
}

pub fn cmp_fields(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &[Field<PortableForm>],
	b: &[Field<PortableForm>],
) -> bool {
	if a.is_empty() && b.is_empty() {
		return true;
	}
	if a.len() != b.len() {
		return false;
	}

	for (a, b) in a.iter().zip(b.iter()) {
		if a.name() != b.name()
			|| a.type_name() != b.type_name()
			|| a.docs() != b.docs()
			|| !cmp_untracked_symbol(a_types, b_types, a.ty(), b.ty())
		{
			return false;
		}
	}

	true
}

pub fn cmp_variants(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &[Variant<PortableForm>],
	b: &[Variant<PortableForm>],
) -> bool {
	if a.is_empty() && b.is_empty() {
		return true;
	}
	if a.len() != b.len() {
		return false;
	}

	for (a, b) in a.iter().zip(b.iter()) {
		if a.name() != b.name()
			|| a.index() != b.index()
			|| a.docs() != b.docs()
			|| !cmp_fields(a_types, b_types, a.fields(), b.fields())
		{
			return false;
		}
	}

	true
}
