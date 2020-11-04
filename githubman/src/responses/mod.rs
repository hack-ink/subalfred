pub mod commits;
pub mod releases;
pub mod tags;

// --- crates.io ---
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
	pub login: String,
	pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Label {
	pub name: String,
}
