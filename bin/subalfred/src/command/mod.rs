mod check;
use check::CheckCmd;

mod convert;
use convert::ConvertCmd;

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

mod workspace;
use workspace::WorkspaceCmd;

/// The main CMD of Subalfred.
#[cmd_impl::cmd]
pub(crate) enum Cmd {
	#[command(subcommand)]
	Check,
	#[command(subcommand)]
	Convert,
	#[command(subcommand)]
	Get,
	Hash,
	Key,
	#[command(subcommand)]
	State,
	StorageKey,
	#[command(subcommand)]
	Workspace,
}
