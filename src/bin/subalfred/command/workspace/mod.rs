mod update;
use update::UpdateCmd;

crate::impl_cmd! {
	#[doc="Workspace manager."]
	WorkspaceCmd {
		Update,
	}
}
