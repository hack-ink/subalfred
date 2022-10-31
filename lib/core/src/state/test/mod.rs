// std
use std::fs;
// hack-ink
use super::*;

const CHAIN_SPEC: &str = "src/state/test/.test.json";

macro_rules! init_test_env {
	() => {
		let _test_env =
			(fs::copy("src/state/test/mock-chain-spec.json", CHAIN_SPEC).unwrap(), TestEnv);
	};
}

struct TestEnv;
impl Drop for TestEnv {
	fn drop(&mut self) {
		let _ = fs::remove_file(CHAIN_SPEC);
	}
}

fn assert_eq_pairs(pairs: &[(&str, &str)]) {
	let chain_spec = system::read_file_to_struct::<_, ChainSpec>(CHAIN_SPEC).unwrap();

	pairs.iter().for_each(|&(k, v)| {
		assert_eq!(chain_spec.genesis.raw.top.get(k).map(|v| v.as_str()).unwrap_or_default(), v);
	});
}

#[test]
fn state_insert_should_work() {
	init_test_env!();

	assert_eq_pairs(&[("0x00", "0x00"), ("0x01", "0x01"), ("0x02", "")]);
	insert_pair_to_chain_spec(CHAIN_SPEC, "0x00".into(), "0x01".into()).unwrap();
	insert_pair_to_chain_spec(CHAIN_SPEC, "0x01".into(), "0x01".into()).unwrap();
	insert_pair_to_chain_spec(CHAIN_SPEC, "0x02".into(), "0x02".into()).unwrap();
	assert_eq_pairs(&[("0x00", "0x01"), ("0x01", "0x01"), ("0x02", "0x02")]);
}
