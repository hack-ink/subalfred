mod update;
use update::UpdateCmd;

/// Workspace manager.
#[cmd_impl::cmd]
pub(crate) enum WorkspaceCmd {
	Update,
}
