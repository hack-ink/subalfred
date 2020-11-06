// --- crates.io ---
use serde::Deserialize;
// --- githubman ---
use crate::responses::{Label, User};

#[derive(Debug, Deserialize)]
pub struct Commit {
	pub sha: String,
	pub url: String,
	pub commit: CommitDetail,
}
#[derive(Debug, Deserialize)]
pub struct CommitDetail {
	pub committer: Committer,
}
#[derive(Debug, Deserialize)]
pub struct Committer {
	pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
	pub html_url: String,
	pub title: String,
	pub user: User,
	pub body: String,
	pub merged_at: String,
	pub labels: Vec<Label>,
}
