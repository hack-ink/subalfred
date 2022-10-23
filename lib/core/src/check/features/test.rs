// hack-ink
use super::*;

#[test]
fn check_mock_runtime_should_work() {
	let mut result = check("src/check/features/mock-runtime/Cargo.toml").unwrap();
	let mut expected = {
		let result: &[(&str, &[&str])] = &[
			("std", &["pallet-a", "primitive-a"]),
			("runtime-benchmarks", &["pallet-b"]),
			("try-runtime", &["pallet-c"]),
		];

		result
			.iter()
			.map(|&(a, b)| (a.to_owned(), b.iter().map(ToString::to_string).collect()))
			.collect::<Vec<_>>()
	};

	result.sort();
	expected.sort();

	assert_eq!(result, expected);
}
