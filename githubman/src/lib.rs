pub mod pager;
pub mod requests;
pub mod responses;
pub mod util;

// --- crates.io ---
use isahc::{
	http::{
		header::ACCEPT, request::Builder as RequestBuilder, Method as HttpMethod, Request,
		Response, Uri,
	},
	Body as IsahcBody, HttpClient, HttpClientBuilder,
};
// --- githubman ---
use crate::pager::Pager;

type IsahcRequest<B> = Request<B>;
type IsahcResponse = Response<IsahcBody>;
type IsahcResult<T> = Result<T, isahc::Error>;

pub trait GithubApi<B>
where
	B: Into<IsahcBody>,
{
	const HTTP_METHOD: HttpMethod;
	const PATH: &'static str;
	const ACCEPT: &'static str;

	fn build_uri(&self) -> Uri;

	fn build_body(&self) -> B;

	fn build_request(&self) -> IsahcRequest<B> {
		let mut request_builder = RequestBuilder::new()
			.method(Self::HTTP_METHOD)
			.uri(self.build_uri());

		request_builder
			.headers_mut()
			.unwrap()
			.append(ACCEPT, Self::ACCEPT.parse().unwrap());

		request_builder.body(self.build_body()).unwrap()
	}

	fn build_request_with_extra_queries(&self, extra_queries: impl AsRef<str>) -> IsahcRequest<B> {
		let uri = self.build_uri();
		let uri = if uri.query().is_none() {
			format!("{}?{}", uri, extra_queries.as_ref())
		} else {
			format!("{}&{}", uri, extra_queries.as_ref())
		};
		let mut request_builder = RequestBuilder::new().method(Self::HTTP_METHOD).uri(uri);

		request_builder
			.headers_mut()
			.unwrap()
			.append(ACCEPT, Self::ACCEPT.parse().unwrap());

		request_builder.body(self.build_body()).unwrap()
	}
}

#[derive(Debug)]
pub struct GithubMan {
	pub http_client: HttpClient,
}
impl GithubMan {
	pub const API_BASE_URL: &'static str = "https://api.github.com";

	pub fn new(oauth_token: &str) -> Self {
		let http_client = HttpClientBuilder::new()
			.default_header("Authorization", &format!("token {}", oauth_token))
			.build()
			.unwrap();

		Self { http_client }
	}

	pub async fn get(&self, request: impl GithubApi<()>) -> IsahcResult<IsahcResponse> {
		let request = request.build_request();

		#[cfg(feature = "dbg")]
		dbg!(request.uri());

		self.http_client.send_async(request).await
	}

	pub async fn get_with_pager(
		&self,
		request: impl GithubApi<()>,
		pager: &mut Pager,
	) -> IsahcResult<IsahcResponse> {
		let request = request.build_request_with_extra_queries(pager.query());

		#[cfg(feature = "dbg")]
		dbg!(request.uri());

		pager.page += 1;

		self.http_client.send_async(request).await
	}
}
