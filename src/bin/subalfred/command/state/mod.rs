mod diff;
use diff::DiffCmd;

mod export;
use export::ExportCmd;

mod fork_off;
use fork_off::ForkOffCmd;

mod r#override;
use r#override::OverrideCmd;

/// A set of tools to process Substrate-like node state.
#[cmd_impl::cmd]
pub(crate) enum StateCmd {
	Diff,
	Export,
	ForkOff,
	Override,
}
