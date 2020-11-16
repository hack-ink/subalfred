// --- crates.io ---
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tag {
	pub name: String,
	pub commit: Commit,
}
#[derive(Debug, Deserialize)]
pub struct Commit {
	pub sha: String,
	pub url: String,
}
