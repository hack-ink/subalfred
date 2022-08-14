mod check;
use check::CheckCmd;

mod convert;
use convert::ConvertCmd;

mod export_state;
use export_state::ExportStateCmd;

mod get;
use get::GetCmd;

mod hash;
use hash::HashCmd;

mod key;
use key::KeyCmd;

mod state;
use state::StateCmd;

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
		$(#[$outer:meta])*
		$cmd:ident {
			$(
				$(#[$inner:meta])*
				$subcmd:ident
			),*,
		}
	) => {
		$(#[$outer])*
		#[derive(Debug, clap::Subcommand)]
		pub(crate) enum $cmd {
			$(
				$(#[$inner])*
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
		#[clap(subcommand)]
		Convert,
		ExportState,
		#[clap(subcommand)]
		Get,
		Hash,
		Key,
		#[clap(subcommand)]
		State,
		StorageKey,
		#[clap(subcommand)]
		Workspace,
	}
}
