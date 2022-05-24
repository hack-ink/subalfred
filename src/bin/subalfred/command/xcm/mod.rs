mod sovereign_address;
use sovereign_address::SovereignAddressCmd;

// crates.io
use clap::Subcommand;
// hack-ink
use crate::prelude::*;

/// XCM utilities.
#[derive(Debug, Subcommand)]
pub enum XcmCmd {
	SovereignAddress(SovereignAddressCmd),
}
impl XcmCmd {
	pub fn run(&self) -> AnyResult<()> {
		match self {
			Self::SovereignAddress(cmd) => cmd.run(),
		}
	}
}
