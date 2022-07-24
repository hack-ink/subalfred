mod runtime_upgrade_block;
use runtime_upgrade_block::RuntimeUpgradeBlockCmd;

crate::impl_cmd! {
	#[doc="Get something from the node."]
	GetCmd {
		RuntimeUpgradeBlock,
	}
}
