mod account;
use account::AccountCmd;

mod check;
use check::CheckCmd;

// mod hash;
// use hash::HashCmd;

// mod rpc;
// use rpc::RpcCmd;

// mod storage_key;
// use storage_key::StorageKeyCmd;

// mod tags;
// use tags::TagsCmd;

// crates.io
use clap::Parser;
// hack-ink
use crate::{AnyResult, Run};

macro_rules! impl_subcommand {
	($($(#[$meta:meta])?$cmd:ident),*,) => {
		#[derive(Debug, Parser)]
		pub enum Subcommand {
			$(
				$(#[$meta])?
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
	};
}

// impl_subcommand![Account, Check, Hash, Rpc, StorageKey, Tags];
impl_subcommand![
	Account,
	#[clap(subcommand)]
	Check,
];
