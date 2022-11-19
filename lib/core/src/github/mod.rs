//! Subalfred core GitHub library.

// std
use std::{env, sync::Arc};
// crates.io
use futures::{stream, StreamExt};
use githuber::{
	api::{commits, Method::*},
	prelude::*,
};
use reqwest::{
	header::{ACCEPT, USER_AGENT},
	Client,
};
use serde::{de::DeserializeOwned, Deserialize};
// hack-ink
use crate::{http::CLIENT, prelude::*};

/// GitHub REST API client.
#[derive(Debug, Clone)]
pub struct ApiClient {
	inner: Arc<Client>,
	token: String,
}
impl ApiClient {
	const PER_PAGE: u8 = 100;
	const USER_AGENT: &'static str = "subalfred-v0.9.0-rc16";

	/// Create a new API client.
	pub fn new() -> Result<Self> {
		Ok(Self { inner: CLIENT.clone(), token: get_github_token()? })
	}

	/// Send the given request.
	pub async fn request<R, D>(&self, request: &R) -> Result<D>
	where
		R: ApiExt,
		D: DeserializeOwned,
	{
		let api = request.api();
		let payload_params = request.payload_params();

		tracing::trace!("Request({api}),Payload({payload_params:?})");

		let response = match R::METHOD {
			Delete => todo!(),
			Get => self
				.inner
				.get(api)
				.header(ACCEPT, R::ACCEPT)
				.header(USER_AGENT, Self::USER_AGENT)
				.bearer_auth(&self.token)
				.query(&payload_params),
			Patch => todo!(),
			Post => todo!(),
			Put => todo!(),
		}
		.send()
		.await
		.map_err(error::Generic::Reqwest)?
		.json::<D>()
		.await
		.map_err(error::Generic::Reqwest)?;

		Ok(response)
	}

	/// Send the given request.
	///
	/// This function is infallible.
	/// It will retry at most `u16::MAX` times.
	/// If the target server is down or the deserialization fails current thread will be blocked.
	pub async fn request_auto_retry<R, D>(&self, request: &R) -> D
	where
		R: Clone + ApiExt,
		D: DeserializeOwned,
	{
		for i in 0_u16.. {
			match self.request::<R, D>(request).await {
				Ok(r) => return r,
				Err(e) => {
					tracing::warn!("request failed dut to: {e:?}, retried {i} times");
				},
			}
		}

		unreachable!("[core::github] there is an infinity loop before; hence this block can never be reached; qed")
	}
}

#[derive(Debug, Deserialize)]
struct Commits {
	commits: Vec<Commit>,
}
#[derive(Debug, Deserialize)]
struct Commit {
	sha: String,
}

/// Elementary pull request type.
#[derive(Debug, Deserialize)]
pub struct PullRequest {
	/// Pull request's title.
	pub title: String,
	/// Pull request's uri.
	pub html_url: String,
	/// Pull request's labels.
	pub labels: Vec<Label>,
}
/// Elementary label type.
#[derive(Debug, Deserialize)]
pub struct Label {
	/// Label's name
	pub name: String,
}

/// Track the updates.
///
/// Basically, it compares two commits and return the associated pull requests.
pub async fn track_update(owner: &str, repo: &str, basehead: &str) -> Result<Vec<PullRequest>> {
	let api_client = ApiClient::new()?;
	let mut request =
		commits::compare_two_commits(owner, repo, basehead).per_page(ApiClient::PER_PAGE).page(1);
	let mut commit_shas = Vec::new();

	loop {
		let response = api_client.request_auto_retry::<_, Commits>(&request).await;
		let page = request
			.page
			.take()
			.expect("[core::github] `page` has already been set in previous step; qed");
		let commits_count = response.commits.len() as u8;

		response.commits.into_iter().for_each(|commit| commit_shas.push(commit.sha));

		if commits_count < ApiClient::PER_PAGE {
			break;
		}

		request = request.page(page + 1);
	}

	let mut pull_requests =
		stream::iter(commit_shas.into_iter().enumerate().map(|(i, commit_sha)| {
			let api_client = api_client.clone();

			async move {
				(
					i,
					api_client
						.request_auto_retry::<_, Vec<PullRequest>>(
							&commits::list_pull_requests_associated_with_a_commit(
								owner,
								repo,
								&commit_sha,
							),
						)
						.await,
				)
			}
		}))
		// TODO: configurable
		.buffer_unordered(32)
		.collect::<Vec<_>>()
		.await;

	pull_requests.sort_by_key(|(i, _)| *i);

	Ok(pull_requests.into_iter().flat_map(|(_, pull_request)| pull_request).collect())
}

fn get_github_token() -> Result<String> {
	Ok(env::var("GITHUB_TOKEN").map_err(error::Github::NoTokenFound)?)
}
