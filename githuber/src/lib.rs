#![allow(clippy::tabs_in_doc_comments)]

pub mod pager;
pub mod requests;
pub mod responses;
pub mod util;

// --- std ---
use std::fmt::Debug;
// --- crates.io ---
use isahc::{
	http::{
		header::ACCEPT, request::Builder as RequestBuilder, Method as HttpMethod, Request,
		Response, Uri,
	},
	Body as IsahcBody, HttpClient, HttpClientBuilder,
};
use tracing::trace;
// --- githuber ---
use crate::pager::Pager;

type IsahcRequest<B> = Request<B>;
type IsahcResponse = Response<IsahcBody>;
type IsahcResult<T> = Result<T, isahc::Error>;

pub trait GithubApi<B>: Clone + Debug
where
	B: Into<IsahcBody>,
{
	const HTTP_METHOD: HttpMethod;
	const PATH: &'static str;
	const ACCEPT: &'static str;

	fn build_uri(&self) -> Uri;

	fn build_body(&self) -> B;

	fn build_request(&self) -> IsahcRequest<B> {
		RequestBuilder::new()
			.method(Self::HTTP_METHOD)
			.header(ACCEPT, Self::ACCEPT)
			.uri(self.build_uri())
			.body(self.build_body())
			.unwrap()
	}

	fn build_request_with_extra_queries(&self, extra_queries: impl AsRef<str>) -> IsahcRequest<B> {
		let uri = self.build_uri();
		let uri = if uri.query().is_none() {
			format!("{}?{}", uri, extra_queries.as_ref())
		} else {
			format!("{}&{}", uri, extra_queries.as_ref())
		};

		RequestBuilder::new()
			.method(Self::HTTP_METHOD)
			.header(ACCEPT, Self::ACCEPT)
			.uri(uri)
			.body(self.build_body())
			.unwrap()
	}
}

#[derive(Clone, Debug)]
pub struct Githuber {
	pub http_client: HttpClient,
}
impl Githuber {
	pub const API_BASE_URL: &'static str = "https://api.github.com";

	pub fn new(oauth_token: impl AsRef<str>) -> Self {
		let http_client = HttpClientBuilder::new()
			.default_header("Authorization", &format!("token {}", oauth_token.as_ref()))
			.build()
			.unwrap();

		Self { http_client }
	}

	pub async fn download(&self, url: impl AsRef<str>) -> IsahcResult<IsahcResponse> {
		let response = self.http_client.get_async(url.as_ref()).await?;

		trace!("{:#?}", response);

		Ok(response)
	}

	pub async fn send<B>(&self, request: impl GithubApi<B>) -> IsahcResult<IsahcResponse>
	where
		B: Into<IsahcBody>,
	{
		let request = request.build_request();

		trace!("{}", request.uri());

		self.http_client.send_async(request).await
	}

	pub async fn send_with_pager<B>(
		&self,
		request: impl GithubApi<B>,
		pager: &mut Pager,
	) -> IsahcResult<IsahcResponse>
	where
		B: Into<IsahcBody>,
	{
		let request = request.build_request_with_extra_queries(pager.query());

		trace!("{}", request.uri());

		pager.page += 1;

		self.http_client.send_async(request).await
	}
}
