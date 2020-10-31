// --- crates.io ---
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{request::Builder as RequestBuilder, Method as HttpMethod};
// --- githubman ---
use crate::{api_queries, GithubApi};

type IsahcRequest = isahc::http::Request<()>;

#[derive(Debug, Default, DeriveBuilder)]
pub struct ListCommits {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// sha	string	query
	/// SHA or branch to start listing commits from. Default: the repository’s default branch (usually master).
	#[builder(default, setter(into, strip_option))]
	pub sha: Option<String>,
	/// path	string	query
	/// Only commits containing this file path will be returned.
	#[builder(default, setter(into, strip_option))]
	pub path: Option<String>,
	/// author	string	query
	/// GitHub login or email address by which to filter by commit author.
	#[builder(default, setter(into, strip_option))]
	pub author: Option<String>,
	/// since	string	query
	/// Only show notifications updated after the given time. This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
	#[builder(default, setter(into, strip_option))]
	pub since: Option<String>,
	/// until	string	query
	/// Only commits before this date will be returned. This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
	#[builder(default, setter(into, strip_option))]
	pub until: Option<String>,
	/// per_page	integer	query
	/// Results per page (max 100)
	#[builder(default, setter(into, strip_option))]
	pub per_page: Option<u32>,
	/// page	integer	query
	/// Page number of the results to fetch.
	#[builder(default, setter(into, strip_option))]
	pub page: Option<u32>,
}
impl Into<IsahcRequest> for ListCommits {
	fn into(self) -> IsahcRequest {
		let uri = api_queries!(
			self,
			[owner, repo],
			[sha, path, author, since, until, per_page, page]
		);

		#[cfg(feature = "dbg")]
		dbg!(&uri);

		RequestBuilder::new()
			.method(Self::HTTP_METHOD)
			.uri(uri)
			.body(())
			.unwrap()
	}
}
impl GithubApi<()> for ListCommits {
	const HTTP_METHOD: HttpMethod = HttpMethod::GET;
	const PATH: &'static str = "/repos/{owner}/{repo}/commits";
}
