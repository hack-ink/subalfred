pub mod commits;

// --- crates.io ---
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Label {
	pub name: String,
}
