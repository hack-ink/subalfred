// --- crates.io ---
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{request::Builder as RequestBuilder, Method as HttpMethod};
// --- githubman ---
use crate::{api_queries, GithubApi};

type IsahcRequest = isahc::http::Request<()>;

#[derive(Debug, Default, DeriveBuilder)]
pub struct ListPullRequestsAssociatedWithACommit {
	// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	// commit_sha	string	path
	// commit_sha+ parameter
	#[builder(setter(into))]
	pub commit_sha: String,
	// per_page	integer	query
	// Results per page (max 100)
	#[builder(default, setter(into, strip_option))]
	pub per_page: Option<u32>,
	// page	integer	query
	// Page number of the results to fetch.
	#[builder(default, setter(into, strip_option))]
	pub page: Option<u32>,
}
impl Into<IsahcRequest> for ListPullRequestsAssociatedWithACommit {
	fn into(self) -> IsahcRequest {
		let uri = api_queries!(self, [owner, repo, commit_sha], [per_page, page]);

		#[cfg(feature = "dbg")]
		dbg!(&uri);

		RequestBuilder::new()
			.method(Self::HTTP_METHOD)
			.uri(uri)
			.body(())
			.unwrap()
	}
}
impl GithubApi<()> for ListPullRequestsAssociatedWithACommit {
	const HTTP_METHOD: HttpMethod = HttpMethod::GET;
	const PATH: &'static str = "/repos/{owner}/{repo}/commits/{commit_sha}/pulls";
}
