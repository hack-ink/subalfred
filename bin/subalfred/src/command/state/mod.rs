mod diff;
use diff::DiffCmd;

mod export;
use export::ExportCmd;

mod fork_off;
use fork_off::ForkOffCmd;

mod insert;
use insert::InsertCmd;

mod r#override;
use r#override::OverrideCmd;

/// A set of tools to process Substrate-like chain state.
#[cmd_impl::cmd]
pub(crate) enum StateCmd {
	Diff,
	Export,
	ForkOff,
	Insert,
	Override,
}
