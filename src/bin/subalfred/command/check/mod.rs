mod std_feature;
use std_feature::StdFeatureCmd;

mod runtime;
use runtime::RuntimeCmd;

/// Some checking tools are pretty useful for runtime development.
#[cmd_impl::cmd]
pub(crate) enum CheckCmd {
	Runtime,
	StdFeature,
}
