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
