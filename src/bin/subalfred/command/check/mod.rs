mod default_features;
use default_features::DefaultFeaturesCmd;

mod runtime;
use runtime::RuntimeCmd;

// crates.io
use clap::Subcommand;
// hack-ink
use crate::{impl_cmd, prelude::*};

impl_cmd! {
	name: CheckCmd,
	DefaultFeatures,
	Runtime,
}
