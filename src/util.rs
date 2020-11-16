pub fn hex(s: &[u8]) -> String {
	s.iter()
		.map(|c| format!("{:02x}", c))
		.collect::<Vec<_>>()
		.join("")
}

pub fn bytes(s: &str) -> Vec<u8> {
	let s = s.trim_start_matches("0x");

	(0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
		.collect()
}
