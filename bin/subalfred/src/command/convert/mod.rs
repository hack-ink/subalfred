mod ascii2hex;
use ascii2hex::Ascii2HexCmd;

mod bytes_style;
use bytes_style::BytesStyleCmd;

mod bytes2hex;
use bytes2hex::Bytes2HexCmd;

mod hex2bytes;
use hex2bytes::Hex2BytesCmd;

/// Converter.
#[cmd_impl::cmd]
pub(crate) enum ConvertCmd {
	#[command(name = "ascii2hex")]
	Ascii2Hex,
	BytesStyle,
	#[command(name = "bytes2hex")]
	Bytes2Hex,
	#[command(name = "hex2bytes")]
	Hex2Bytes,
}
