// crates.io
use vergen::{Config, ShaKind};

fn main() {
	let mut config = Config::default();

	*config.git_mut().sha_kind_mut() = ShaKind::Short;

	match vergen(config) {
		Ok(_) => (),
		// Disable the git version if installed from <crates.io>.
		Err(e) => {
			*config.git_mut().enabled_mut() = false;

			vergen(config).unwrap();
		},
	}
}
