// --- crates.io ---
use serde::Deserialize;
// --- githubman ---
use crate::responses::Label;

#[derive(Debug, Deserialize)]
pub struct Commit {
	pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
	pub url: String,
	pub labels: Vec<Label>,
}
