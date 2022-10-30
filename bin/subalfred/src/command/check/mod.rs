mod features;
use features::FeaturesCmd;

mod runtime;
use runtime::RuntimeCmd;

/// Substrate development checkers.
#[cmd_impl::cmd]
pub(crate) enum CheckCmd {
	Runtime,
	Features,
}
