mod sovereign_address;
use sovereign_address::SovereignAddressCmd;

crate::impl_cmd! {
	#[doc="XCM utilities."]
	XcmCmd {
		SovereignAddress,
	}
}
