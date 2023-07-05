// std
use std::path::PathBuf;
// subalfred
use super::*;

#[test]
fn swap_file_path_should_work() {
	assert_eq!(
		["/subalfred/Cargo.toml", "/subalfred/substrate-minimal/subrpcer/impl/Cargo.toml"]
			.iter()
			.map(|path| swapped_file_path(path).unwrap())
			.collect::<Vec<_>>(),
		vec![
			PathBuf::from("/subalfred/.Cargo.toml.swp"),
			PathBuf::from("/subalfred/substrate-minimal/subrpcer/impl/.Cargo.toml.swp"),
		]
	);
}
