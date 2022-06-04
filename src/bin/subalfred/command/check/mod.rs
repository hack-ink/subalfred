mod std_feature;
use std_feature::StdFeatureCmd;

mod runtime;
use runtime::RuntimeCmd;

crate::impl_cmd! {
	#[doc="Some checking tools are pretty useful for runtime development."]
	CheckCmd {
		Runtime,
		StdFeature,
	}
}
