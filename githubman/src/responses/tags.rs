// --- crates.io ---
use serde::Deserialize;
// --- githubman ---
use crate::responses::commits::Commit;

#[derive(Debug, Deserialize)]
pub struct Tag {
	pub name: String,
	pub commit: Commit,
}
