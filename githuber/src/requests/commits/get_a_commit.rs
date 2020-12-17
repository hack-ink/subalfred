// --- crates.io ---
use derive_builder::Builder as DeriveBuilder;
use isahc::http::{Method as HttpMethod, Uri};
// --- githuber ---
use crate::{uri, GithubApi};

#[derive(Clone, Debug, Default, DeriveBuilder)]
pub struct GetACommit {
	/// owner	string	path
	#[builder(setter(into))]
	pub owner: String,
	/// repo	string	path
	#[builder(setter(into))]
	pub repo: String,
	/// ref	string	path
	/// ref+ parameter
	#[builder(setter(into))]
	pub r#ref: String,
}
impl GithubApi<()> for GetACommit {
	const HTTP_METHOD: HttpMethod = HttpMethod::GET;
	const PATH: &'static str = "/repos/{owner}/{repo}/commits/{ref}";
	const ACCEPT: &'static str = "application/vnd.github.v3+json";

	fn build_uri(&self) -> Uri {
		uri!(self, [owner, repo, r#ref]).parse().unwrap()
	}

	fn build_body(&self) {}
}
