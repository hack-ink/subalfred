// hack-ink
use super::*;

#[test]
fn check_mock_runtime_should_work() {
	assert_eq!(
		check("src/subalfred/core/check/features/mock-runtime/Cargo.toml").unwrap(),
		vec![
			("std".to_string(), vec!["pallet-a".to_string(), "primitive-a".to_string()]),
			("runtime-benchmarks".to_string(), vec!["pallet-b".to_string()]),
			("try-runtime".to_string(), vec!["pallet-c".to_string()]),
		]
	);
}
