// crates.io
use cargo_metadata::{Metadata, MetadataCommand};
// hack-ink
use super::*;

fn test_data() -> Metadata {
	MetadataCommand::new().manifest_path("./Cargo.toml").exec().unwrap()
}

#[test]
fn members_manifest_paths_should_work() {
	fn remove_common_prefix(s: &String) -> &str {
		s.split_once("/subalfred").unwrap().1
	}

	let metadata = test_data();

	assert_eq!(
		members_manifest_paths(&metadata).iter().map(remove_common_prefix).collect::<Vec<_>>(),
		vec![
			"/substrate-minimum/subcryptor/Cargo.toml",
			"/substrate-minimum/subgrandpa/Cargo.toml",
			"/substrate-minimum/subhasher/Cargo.toml",
			"/substrate-minimum/submetadatan/Cargo.toml",
			"/substrate-minimum/subrpcer/Cargo.toml",
			"/substrate-minimum/subrpcer/impl/Cargo.toml",
			"/substrate-minimum/substorager/Cargo.toml",
			"/substrate-minimum/subversion/Cargo.toml",
			"/Cargo.toml"
		]
	);
}
