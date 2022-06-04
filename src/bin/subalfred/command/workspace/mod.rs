mod update;
use update::{UpdateCmd, UpdateDepsCmd};

crate::impl_cmd! {
	#[doc="Workspace manager."]
	WorkspaceCmd {
		Update,
		UpdateDeps,
	}
}
