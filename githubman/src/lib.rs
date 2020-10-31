pub mod action;
pub mod util;

// --- crates.io ---
use isahc::{http::Method as HttpMethod, Body as HttpBody, HttpClient, HttpClientBuilder};

type IsahcRequest<B> = isahc::http::Request<B>;
type IsahcResponse = isahc::http::Response<HttpBody>;
type IsahcResult<T> = Result<T, isahc::Error>;

// TODO: configurable
const OAUTH_TOKEN: &'static str = "b8aafd15d8ae950ba8425da60df5329a6bb1ea7a";

pub trait GithubApi<B>: Into<IsahcRequest<B>>
where
	B: Into<HttpBody>,
{
	const HTTP_METHOD: HttpMethod;
	const PATH: &'static str;
}

#[derive(Debug)]
pub struct GithubMan {
	pub http_client: HttpClient,
}
impl GithubMan {
	pub const API_BASE_URL: &'static str = "https://api.github.com";

	pub fn new() -> Self {
		let http_client = HttpClientBuilder::new()
			.default_headers(&[
				("accept", "application/vnd.github.v3+json"),
				("Authorization", &format!("token {}", OAUTH_TOKEN)),
			])
			.build()
			.unwrap();

		Self { http_client }
	}

	pub async fn request<B>(&self, request: impl GithubApi<B>) -> IsahcResult<IsahcResponse>
	where
		B: Into<HttpBody>,
	{
		self.http_client.send_async(request.into()).await
	}
}
