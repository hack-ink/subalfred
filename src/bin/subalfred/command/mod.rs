mod address;
use address::AddressCmd;

mod storage_key;
use storage_key::StorageKeyCmd;

mod check;
use check::CheckCmd;

mod xcm;
use xcm::XcmCmd;

// crates.io
use clap::Subcommand;
// hack-ink
use crate::prelude::*;

#[macro_export]
macro_rules! impl_cmd {
	(name: $cmd:ident,$($(#[$meta:meta])?$subcmd:ident),*,) => {
		#[derive(Debug, Subcommand)]
		pub enum $cmd {
			$(
				$(#[$meta])?
				$subcmd(concat_idents!($subcmd, Cmd))
			),*
		}
		impl $cmd {
			pub fn run(&self) -> AnyResult<()> {
				match self {
					$(
						Self::$subcmd(subcmd) => { subcmd.run() }
					),*
				}
			}
		}
	};
}

impl_cmd! {
	name: Cmd,
	Address,
	StorageKey,
	#[clap(subcommand)]
	Xcm,
	#[clap(subcommand)]
	Check,
}
