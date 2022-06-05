// std
use std::borrow::Cow;
// crates.io
use cargo_metadata::{Metadata, Package, PackageId};

#[derive(Debug)]
pub enum VersionSpec {
	Majored = 0,
	Minored,
	Patched,
}
impl From<&str> for VersionSpec {
	fn from(s: &str) -> Self {
		match s.chars().fold(0_u8, |acc, c| if c == '.' { acc + 1 } else { acc }) {
			0 => Self::Majored,
			1 => Self::Minored,
			2 => Self::Patched,
			_ => Self::Patched,
		}
	}
}

pub fn find_package<'a>(metadata: &'a Metadata, id: &PackageId) -> Option<&'a Package> {
	metadata.packages.iter().find(|pkg| &pkg.id == id)
}

pub fn align_version<'a>(from: &str, to: &'a str) -> Cow<'a, str> {
	let from_spec = VersionSpec::from(from);
	let to_spec = VersionSpec::from(to);

	match from_spec as i8 - to_spec as i8 {
		-2 => Cow::Owned(to.split_once('.').unwrap_or_default().0.into()),
		-1 => Cow::Owned(to.rsplit_once('.').unwrap_or_default().0.into()),
		0 => Cow::Borrowed(to),
		1 => Cow::Owned(format!("{to}.0")),
		2 => Cow::Owned(format!("{to}.0.0")),
		_ => Cow::Borrowed(to),
	}
}
