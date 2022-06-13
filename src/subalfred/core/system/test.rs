// hack-ink
use super::*;

#[test]
fn swap_file_path_should_work() {
	assert_eq!(
		["/subalfred/Cargo.toml", "/subalfred/substrate-minimum/subrpcer/impl/Cargo.toml",]
			.iter()
			.map(|path| swapped_file_path(path).unwrap())
			.collect::<Vec<_>>(),
		vec![
			Utf8PathBuf::from("/subalfred/.Cargo.toml.swp"),
			Utf8PathBuf::from("/subalfred/substrate-minimum/subrpcer/impl/.Cargo.toml.swp"),
		]
	);
}
