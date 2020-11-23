pub fn hash(data: &str, hasher: &str, is_hex: bool) -> String {
	let (data, bytes) = if is_hex {
		let data = data.trim_start_matches("0x");

		(
			format!(r#"array_bytes::hex_str("0x{}")"#, data),
			array_bytes::bytes(data).unwrap(),
		)
	} else {
		(format!(r#""{}""#, data), data.as_bytes().to_vec())
	};

	match hasher {
		"blake2-128" => format!(
			"subhasher::blake2_128({}) = {}",
			data,
			array_bytes::hex_str(&subhasher::blake2_128(&bytes))
		),
		"blake2-256" => format!(
			"subhasher::blake2_256({}) = {}",
			data,
			array_bytes::hex_str(&subhasher::blake2_256(&bytes))
		),
		"blake2-128-concat" => format!(
			"subhasher::blake2_128_concat({}) = {}",
			data,
			array_bytes::hex_str(&subhasher::blake2_128_concat(&bytes))
		),
		"twox-64" => format!(
			"subhasher::twox_64({}) = {}",
			data,
			array_bytes::hex_str(&subhasher::twox_64(&bytes))
		),
		"twox-128" => format!(
			"subhasher::twox_128({}) = {}",
			data,
			array_bytes::hex_str(&subhasher::twox_128(&bytes))
		),
		"twox-256" => format!(
			"twox256({}) = {}",
			data,
			array_bytes::hex_str(&subhasher::twox_256(&bytes))
		),
		"twox-128-concat" => format!(
			"twox_128_concat({}) = {}",
			data,
			array_bytes::hex_str(&subhasher::twox_64_concat(&bytes))
		),
		"identity" => format!("identity({}) = {}", data, array_bytes::hex_str(&bytes)),
		_ => unreachable!(),
	}
}

pub fn parse_storage_keys(module: Option<&str>, item: Option<&str>) -> String {
	let mut storage_prefix = String::from("0x");

	if let Some(module) = module {
		storage_prefix.push_str(&array_bytes::hex_str(&subhasher::twox_128(
			module.as_bytes(),
		)));
	}
	if let Some(item) = item {
		storage_prefix.push_str(&array_bytes::hex_str(&subhasher::twox_128(item.as_bytes())));
	}

	storage_prefix
}
