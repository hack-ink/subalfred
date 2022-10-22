mod features;
use features::FeaturesCmd;

mod runtime;
use runtime::RuntimeCmd;

/// Some checking tools are pretty useful for runtime development.
#[cmd_impl::cmd]
pub(crate) enum CheckCmd {
	Runtime,
	Features,
}
