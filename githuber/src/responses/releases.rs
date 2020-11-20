// --- crates.io ---
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Release {
	pub html_url: String,
	pub tag_name: String,
	pub target_commitish: String,
	pub name: String,
	pub prerelease: bool,
	pub created_at: String,
	pub published_at: String,
	pub body: String,
}
