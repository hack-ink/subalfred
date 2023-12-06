mod shared;

mod check;
use check::CheckCmd;

mod convert;
use convert::ConvertCmd;

mod decrypt;
use decrypt::DecryptCmd;

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

mod track_updates;
use track_updates::TrackUpdatesCmd;

mod workspace;
use workspace::WorkspaceCmd;

/// The primary directive of Subalfred.
#[cmd_impl::cmd]
pub(crate) enum Cmd {
	#[command(subcommand)]
	Check,
	#[command(subcommand)]
	Convert,
	Decrypt,
	#[command(subcommand)]
	Get,
	Hash,
	Key,
	Rpc,
	#[command(subcommand)]
	State,
	StorageKey,
	TrackUpdates,
	#[command(subcommand)]
	Workspace,
}
