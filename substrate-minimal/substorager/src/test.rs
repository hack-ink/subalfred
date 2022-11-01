// hack-ink
use super::*;

#[test]
fn storage_key_should_work() {
	assert_eq!(
		storage_key(b"System", b"Account").0,
		[
			38, 170, 57, 78, 234, 86, 48, 224, 124, 72, 174, 12, 149, 88, 206, 247, 185, 157, 136,
			14, 198, 129, 121, 156, 12, 243, 14, 136, 134, 55, 29, 169
		]
	);
}
