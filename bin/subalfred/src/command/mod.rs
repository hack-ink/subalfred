mod shared;

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

mod rpc;
use rpc::RpcCmd;

mod state;
use state::StateCmd;

mod storage_key;
use storage_key::StorageKeyCmd;

mod track_update;
use track_update::TrackUpdateCmd;

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
	Rpc,
	#[command(subcommand)]
	State,
	StorageKey,
	TrackUpdate,
	#[command(subcommand)]
	Workspace,
}
