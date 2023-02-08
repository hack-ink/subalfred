//! Subalfred core GitHub library.

#[cfg(test)] mod test;

pub mod substrate;

// std
use std::{env, sync::Arc};
// crates.io
use githuber::api::{ApiExt, Method::*};
use reqwest::{
	header::{ACCEPT, USER_AGENT},
	Client,
};
use serde::{de::DeserializeOwned, Deserialize};
// subalfred
use crate::{http::CLIENT, prelude::*};

/// GitHub REST API client.
#[derive(Debug, Clone)]
pub struct ApiClient {
	inner: Arc<Client>,
	token: String,
}
impl ApiClient {
	const PER_PAGE: u8 = 100;
	const USER_AGENT: &'static str = "subalfred-api-client";

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
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Clone, Debug, Deserialize)]
pub struct PullRequest {
	/// Pull request's title.
	pub title: String,
	/// Pull request's uri.
	pub html_url: String,
	/// Pull request's labels.
	pub labels: Vec<Label>,
}
/// Elementary label type.
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Clone, Debug, Deserialize)]
pub struct Label {
	/// Label's name
	pub name: String,
}

fn get_github_token() -> Result<String> {
	Ok(env::var("GITHUB_TOKEN").map_err(error::Github::NoTokenFound)?)
}
