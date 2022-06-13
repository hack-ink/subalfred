// crates.io
use cargo_metadata::{Metadata, MetadataCommand};
// hack-ink
use super::*;

fn test_data() -> Metadata {
	MetadataCommand::new().manifest_path("./Cargo.toml").exec().unwrap()
}

#[test]
fn members_manifests_should_work() {
	fn remove_common_prefix(s: &str) -> String {
		s.split_once("/subalfred").unwrap().1.to_owned()
	}

	let metadata = test_data();

	assert_eq!(
		members(&metadata)
			.unwrap()
			.iter()
			.map(|pkg| { (pkg.name.as_str(), remove_common_prefix(&pkg.manifest_path.as_str())) })
			.collect::<Vec<_>>(),
		vec![
			("subcryptor", "/substrate-minimum/subcryptor/Cargo.toml".into()),
			("subgrandpa", "/substrate-minimum/subgrandpa/Cargo.toml".into()),
			("subhasher", "/substrate-minimum/subhasher/Cargo.toml".into()),
			("submetadatan", "/substrate-minimum/submetadatan/Cargo.toml".into()),
			("subrpcer", "/substrate-minimum/subrpcer/Cargo.toml".into()),
			("subrpcer-impl", "/substrate-minimum/subrpcer/impl/Cargo.toml".into()),
			("substorager", "/substrate-minimum/substorager/Cargo.toml".into()),
			("subversion", "/substrate-minimum/subversion/Cargo.toml".into()),
			("subalfred", "/Cargo.toml".into()),
		]
	);
}

#[test]
fn align_version_should_work() {
	["0", "0.0", "0.0.0"].iter().for_each(|from| {
		["1", "1.0", "1.0.0"].iter().for_each(|to| {
			dbg!(from, to);
			match *from {
				"0" => assert_eq!(util::align_version(from, to), "1"),
				"0.0" => assert_eq!(util::align_version(from, to), "1.0"),
				"0.0.0" => assert_eq!(util::align_version(from, to), "1.0.0"),
				_ => unreachable!(),
			}
		});
	});
}
