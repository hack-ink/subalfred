mod account;
use account::AccountCmd;

mod ci;
use ci::CiCmd;

mod hash;
use hash::HashCmd;

mod rpc;
use rpc::RpcCmd;

mod storage_key;
use storage_key::StorageKeyCmd;

// --- crates.io ---
use structopt::StructOpt;
// --- subalfred ---
use crate::{cli::Run, AnyResult};

macro_rules! impl_subcommand {
	($($cmd:ident),*) => {
		#[derive(Debug, StructOpt)]
		pub enum Subcommand {
			$(
				$cmd(concat_idents!($cmd, Cmd))
			),*
		}
		impl Run for Subcommand {
			fn run(&self) -> AnyResult<()> {
				match self {
					$(
						Subcommand::$cmd(cmd) => { cmd.run() }
					),*
				}
			}
		}
	}
}

impl_subcommand![Account, Ci, Hash, Rpc, StorageKey];
