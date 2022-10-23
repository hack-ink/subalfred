mod runtime_upgrade_block;
use runtime_upgrade_block::RuntimeUpgradeBlockCmd;

/// Get something from the node.
#[cmd_impl::cmd]
pub(crate) enum GetCmd {
	RuntimeUpgradeBlock,
}
