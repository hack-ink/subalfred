// --- crates.io ---
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Content {
	pub download_url: String,
}
