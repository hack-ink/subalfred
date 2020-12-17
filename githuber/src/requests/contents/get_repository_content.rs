// --- crates.io ---
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{Method as HttpMethod, Uri};
// --- githuber ---
use crate::{uri, GithubApi};

#[derive(Clone, Debug, Default, DeriveBuilder)]
pub struct GetRepositoryContent {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// path	string	path
	/// path+ parameter
	#[builder(setter(into))]
	pub path: String,
	/// ref	string	query
	/// The name of the commit/branch/tag. Default: the repository’s default branch (usually master)
	#[builder(default)]
	pub r#ref: Option<String>,
}
impl GithubApi<()> for GetRepositoryContent {
	const HTTP_METHOD: HttpMethod = HttpMethod::GET;
	const PATH: &'static str = "/repos/{owner}/{repo}/contents/{path}";
	const ACCEPT: &'static str = "application/vnd.github.v3+json";

	fn build_uri(&self) -> Uri {
		uri!(self, [owner, repo, path], [r#ref]).parse().unwrap()
	}

	fn build_body(&self) {}
}
