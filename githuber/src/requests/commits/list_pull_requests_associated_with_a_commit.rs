// --- crates.io ---
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{Method as HttpMethod, Uri};
// --- githuber ---
use crate::{uri, GithubApi};

#[derive(Clone, Debug, Default, DeriveBuilder)]
pub struct ListPullRequestsAssociatedWithACommit {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// commit_sha	string	path
	/// commit_sha+ parameter
	#[builder(setter(into))]
	pub commit_sha: String,
	/// per_page	integer	query
	/// Results per page (max 100)
	#[builder(default)]
	pub per_page: Option<u32>,
	/// page	integer	query
	/// Page number of the results to fetch.
	#[builder(default)]
	pub page: Option<u32>,
}
impl GithubApi<()> for ListPullRequestsAssociatedWithACommit {
	const HTTP_METHOD: HttpMethod = HttpMethod::GET;
	const PATH: &'static str = "/repos/{owner}/{repo}/commits/{commit_sha}/pulls";
	const ACCEPT: &'static str = "application/vnd.github.groot-preview+json";

	fn build_uri(&self) -> Uri {
		uri!(self, [owner, repo, commit_sha], [per_page, page])
			.parse()
			.unwrap()
	}

	fn build_body(&self) {}
}
