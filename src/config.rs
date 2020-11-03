// --- std ---
use std::io::{Read, Write};
// --- crates.io ---
use serde::{Deserialize, Serialize};
// --- subalfred ---
use crate::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	pub github_oauth_token: String,
}
impl Config {
	pub fn from_reader(r: impl Read) -> Result<Self> {
		serde_yaml::from_reader(r).map_err(Into::into)
	}

	pub fn to_writer(&self, w: impl Write) -> Result<()> {
		// TODO
		// serde_yaml::to_writer(w, self).map_err(Into::into)

		const TEMPLATE: &'static [u8] =
br#"# You can use OAuth tokens to interact with GitHub via automated scripts.
#   https://docs.github.com/en/free-pro-team@latest/github/extending-github/git-automation-with-oauth-tokens
# Get your personal.
#   https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token
github-oauth-token: """#;

		let mut w = w;

		w.write_all(TEMPLATE)?;
		w.flush().map_err(Into::into)
	}
}
