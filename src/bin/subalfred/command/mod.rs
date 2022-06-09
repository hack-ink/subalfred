mod check;
use check::CheckCmd;

mod key;
use key::KeyCmd;

mod storage_key;
use storage_key::StorageKeyCmd;

// TODO: move to a single crate
mod workspace;
use workspace::WorkspaceCmd;

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
		#[clap(subcommand)]
		Check,
		Key,
		StorageKey,
		#[clap(subcommand)]
		Workspace,
	}
}
