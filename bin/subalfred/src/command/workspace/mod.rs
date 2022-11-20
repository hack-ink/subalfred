mod update;
use update::UpdateCmd;

mod update_deps;
use update_deps::UpdateDepsCmd;

/// Workspace manager.
#[cmd_impl::cmd]
pub(crate) enum WorkspaceCmd {
	Update,
	UpdateDeps,
}
