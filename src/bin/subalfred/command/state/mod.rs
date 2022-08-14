mod diff;
use diff::DiffCmd;

mod r#override;
use r#override::OverrideCmd;

crate::impl_cmd! {
	#[doc="A set of tools to process Substrate-like node state."]
	StateCmd {
		Diff,
		Override,
	}
}
