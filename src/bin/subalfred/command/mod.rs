mod check;
use check::CheckCmd;

mod export_state;
use export_state::ExportStateCmd;

mod get;
use get::GetCmd;

mod hash;
use hash::HashCmd;

mod key;
use key::KeyCmd;

mod storage_key;
use storage_key::StorageKeyCmd;

// TODO: move to a single crate
mod workspace;
use workspace::WorkspaceCmd;

// TODO: rewrite into attribute macro.
/// Quickly define and implement a command containing serval subcommands.
#[macro_export]
macro_rules! impl_cmd {
	(
		$(#[doc=$doc:expr])?
		$cmd:ident {
			$(
				$(#[clap($clap_attr:ident)])?
				$subcmd:ident
			),*,
		}
	) => {
		$(#[doc=$doc])?
		#[derive(Debug, clap::Subcommand)]
		pub(crate) enum $cmd {
			$(
				$(#[clap($clap_attr)])?
				$subcmd(concat_idents!($subcmd, Cmd))
			),*
		}
		impl $cmd {
			pub(crate) fn run(&self) -> $crate::prelude::Result<()> {
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
	#[doc="The main CMD of Subalfred."]
	Cmd {
		#[clap(subcommand)]
		Check,
		ExportState,
		#[clap(subcommand)]
		Get,
		Hash,
		Key,
		StorageKey,
		#[clap(subcommand)]
		Workspace,
	}
}
