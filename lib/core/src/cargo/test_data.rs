pub(crate) const TOML: &str = r#"
[package]
authors     = ["Xavier Lau <xavier@inv.cafe>"]
description = "Test"
edition     = "2021"
homepage    = "https://subalfred.hack.ink"
license     = "GPL-3.0"
name        = "test"
readme      = "README.md"
repository  = "https://github.com/hack-ink/subalfred"
version     = "0.0.0"

[dependencies]
# crates.io
a = { version = "0" }
b = { version = "0.0" }
c = { version = "0.0.0" }
# cumulus
d = { git = "https://github.com/a/cumulus", branch = "polkadot-v0.0.0" }
# polkadot
e = { git = "https://github.com/b/polkadot", branch = "release-v0.0.0" }
# substrate
f = { git = "https://github.com/c/substrate", branch = "polkadot-v0.0.0" }
"#;

pub(crate) const EXPECTED_1: &str = r#"
[package]
authors     = ["Xavier Lau <xavier@inv.cafe>"]
description = "Test"
edition     = "2021"
homepage    = "https://subalfred.hack.ink"
license     = "GPL-3.0"
name        = "test"
readme      = "README.md"
repository  = "https://github.com/hack-ink/subalfred"
version     = "0.0.0"

[dependencies]
# crates.io
a = { version = "1" }
b = { version = "1.0" }
c = { version = "1.0.0" }
# cumulus
d = { git = "https://github.com/a/cumulus", branch = "polkadot-v0.0.0" }
# polkadot
e = { git = "https://github.com/b/polkadot", branch = "release-v0.0.0" }
# substrate
f = { git = "https://github.com/c/substrate", branch = "polkadot-v0.0.0" }
"#;

pub(crate) const EXPECTED_2: &str = r#"
[package]
authors     = ["Xavier Lau <xavier@inv.cafe>"]
description = "Test"
edition     = "2021"
homepage    = "https://subalfred.hack.ink"
license     = "GPL-3.0"
name        = "test"
readme      = "README.md"
repository  = "https://github.com/hack-ink/subalfred"
version     = "0.0.0"

[dependencies]
# crates.io
a = { version = "0" }
b = { version = "0.0" }
c = { version = "0.0.0" }
# cumulus
d = { git = "https://github.com/a/cumulus", branch = "polkadot-v1.0.0" }
# polkadot
e = { git = "https://github.com/b/polkadot", branch = "release-v1.0.0" }
# substrate
f = { git = "https://github.com/c/substrate", branch = "polkadot-v1.0.0" }
"#;
