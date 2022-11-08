// crates.io
use vergen::{Config, ShaKind};

fn main() {
	let mut config = Config::default();

	*config.git_mut().sha_kind_mut() = ShaKind::Short;

	// Disable the git version if installed from <crates.io>.
	if vergen::vergen(config.clone()).is_err() {
		*config.git_mut().enabled_mut() = false;

		println!("cargo:rustc-env=VERGEN_GIT_SHA_SHORT=crates.io");

		vergen::vergen(config).unwrap();
	}
}
