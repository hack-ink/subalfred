mod runtime_upgrade_block;
use runtime_upgrade_block::RuntimeUpgradeBlockCmd;

/// Substrate-link node getter.
#[cmd_impl::cmd]
pub(crate) enum GetCmd {
	RuntimeUpgradeBlock,
}
