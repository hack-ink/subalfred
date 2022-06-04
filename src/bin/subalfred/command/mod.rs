mod address;
use address::AddressCmd;

mod check;
use check::CheckCmd;

// TODO: move to a single crate
mod workspace;
use workspace::WorkspaceCmd;

mod storage_key;
use storage_key::StorageKeyCmd;

mod xcm;
use xcm::XcmCmd;

// TODO: rewrite into attribute macro.
#[macro_export]
macro_rules! impl_cmd {
	(
		$(#[$doc:meta])*
		$cmd:ident {
			$(
				$(#[$clap_attr:meta])*
				$subcmd:ident
			),*,
		}
	) => {
		$(#[$doc])*
		#[derive(Debug, clap::Subcommand)]
		pub enum $cmd {
			$(
				$(#[$clap_attr])*
				$subcmd(concat_idents!($subcmd, Cmd))
			),*
		}
		impl $cmd {
			pub fn run(&self) -> $crate::prelude::AnyResult<()> {
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
	Cmd {
		Address,
		#[clap(subcommand)]
		Check,
		StorageKey,
		#[clap(subcommand)]
		Workspace,
		#[clap(subcommand)]
		Xcm,
	}
}
