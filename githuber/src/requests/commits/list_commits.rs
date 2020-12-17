// --- crates.io ---
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{Method as HttpMethod, Uri};
// --- githuber ---
use crate::{uri, GithubApi};

#[derive(Clone, Debug, Default, DeriveBuilder)]
pub struct ListCommits {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// sha	string	query
	/// SHA or branch to start listing commits from. Default: the repository’s default branch (usually master).
	#[builder(default)]
	pub sha: Option<String>,
	/// path	string	query
	/// Only commits containing this file path will be returned.
	#[builder(default)]
	pub path: Option<String>,
	/// author	string	query
	/// GitHub login or email address by which to filter by commit author.
	#[builder(default)]
	pub author: Option<String>,
	/// since	string	query
	/// Only show notifications updated after the given time. This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
	#[builder(default)]
	pub since: Option<String>,
	/// until	string	query
	/// Only commits before this date will be returned. This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
	#[builder(default)]
	pub until: Option<String>,
	/// per_page	integer	query
	/// Results per page (max 100)
	#[builder(default)]
	pub per_page: Option<u32>,
	/// page	integer	query
	/// Page number of the results to fetch.
	#[builder(default)]
	pub page: Option<u32>,
}
impl GithubApi<()> for ListCommits {
	const HTTP_METHOD: HttpMethod = HttpMethod::GET;
	const PATH: &'static str = "/repos/{owner}/{repo}/commits";
	const ACCEPT: &'static str = "application/vnd.github.groot-preview+json";

	fn build_uri(&self) -> Uri {
		uri!(
			self,
			[owner, repo],
			[sha, path, author, since, until, per_page, page]
		)
		.parse()
		.unwrap()
	}

	fn build_body(&self) {}
}
