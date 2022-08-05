mod bytes_style;
use bytes_style::BytesStyleCmd;

mod bytes2hex;
use bytes2hex::Bytes2HexCmd;

mod hex2bytes;
use hex2bytes::Hex2BytesCmd;

crate::impl_cmd! {
	#[doc="Converter."]
	ConvertCmd {
		BytesStyle,
		#[clap(name = "bytes2hex")]
		Bytes2Hex,
		#[clap(name = "hex2bytes")]
		Hex2Bytes,
	}
}
