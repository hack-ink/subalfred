//! Collection of Substrate GitHub related functions.

// crates.io
use futures::{stream, StreamExt};
use githuber::api::commits;
// subalfred
use super::*;
use crate::prelude::*;

/// The specific labels that worth to watch.
pub enum WatchedLabels {
	/// [https://github.com/paritytech/substrate/labels/B3-apinoteworthy]
	B3Api,
	/// [https://github.com/paritytech/substrate/labels/B5-clientnoteworthy]
	B5Client,
	/// [https://github.com/paritytech/substrate/labels/B7-runtimenoteworthy]
	B7Runtime,
	/// [https://github.com/paritytech/substrate/labels/C7-high%20❗%EF%B8%8F]
	C7High,
	/// [https://github.com/paritytech/substrate/labels/C9-critical%20‼%EF%B8%8F]
	C9Critical,
	/// [https://github.com/paritytech/substrate/labels/E1-runtimemigration]
	E1Runtime,
	/// [https://github.com/paritytech/substrate/labels/E2-databasemigration]
	E2Database,
	/// [https://github.com/paritytech/substrate/labels/E4-newhostfunctions]
	E4NewHostFunctions,
	/// [https://github.com/paritytech/substrate/labels/E5-breaksapi]
	E5BreaksApi,
	/// [https://github.com/paritytech/substrate/labels/E6-transactionversion]
	E6TransactionVersion,
	/// [https://github.com/paritytech/substrate/labels/E7-breaksauthoring]
	E7BreaksAuthoring,
	/// [https://github.com/paritytech/substrate/labels/E8-breakseverything]
	E8BreaksEverything,
	/// [https://github.com/paritytech/substrate/labels/E10-client-update-first%20👀]
	E10ClientUpdateFirst,
	/// [https://github.com/paritytech/substrate/labels/I8-enhancement%20🎁]
	I8Enhancement,
}
impl WatchedLabels {
	const fn as_str(&self) -> &'static str {
		// subalfred
		use WatchedLabels::*;

		match self {
			B3Api => "B3-apinoteworthy",
			B5Client => "B5-clientnoteworthy",
			B7Runtime => "B7-runtimenoteworthy",
			C7High => "C7-high ❗️",
			C9Critical => "C9-critical ‼️",
			E1Runtime => "E1-runtimemigration",
			E2Database => "E2-databasemigration",
			E4NewHostFunctions => "E4-newhostfunctions",
			E5BreaksApi => "E5-breaksapi",
			E6TransactionVersion => "E6-transactionversion",
			E7BreaksAuthoring => "E7-breaksauthoring",
			E8BreaksEverything => "E8-breakseverything",
			E10ClientUpdateFirst => "E10-client-update-first 👀",
			I8Enhancement => "I8-enhancement 🎁",
		}
	}

	/// Return all labels' name.
	pub fn all() -> Vec<&'static str> {
		// subalfred
		use WatchedLabels::*;

		vec![
			B3Api.as_str(),
			B5Client.as_str(),
			B7Runtime.as_str(),
			C7High.as_str(),
			C9Critical.as_str(),
			E1Runtime.as_str(),
			E2Database.as_str(),
			E4NewHostFunctions.as_str(),
			E5BreaksApi.as_str(),
			E6TransactionVersion.as_str(),
			E7BreaksAuthoring.as_str(),
			E8BreaksEverything.as_str(),
			E10ClientUpdateFirst.as_str(),
			I8Enhancement.as_str(),
		]
	}
}

/// The pull requests with specific labels that worth to watch.
#[derive(Debug, Default)]
pub struct WatchedPullRequests {
	b3_api: Vec<PullRequest>,
	b5_client: Vec<PullRequest>,
	b7_runtime: Vec<PullRequest>,
	c7_high: Vec<PullRequest>,
	c9_critical: Vec<PullRequest>,
	e1_runtime: Vec<PullRequest>,
	e2_database: Vec<PullRequest>,
	e4_new_host_functions: Vec<PullRequest>,
	e5_breaks_api: Vec<PullRequest>,
	e6_transaction_version: Vec<PullRequest>,
	e7_breaks_authoring: Vec<PullRequest>,
	e8_breaks_everything: Vec<PullRequest>,
	e10_client_update_first: Vec<PullRequest>,
	i8_enhancement: Vec<PullRequest>,
}
impl WatchedPullRequests {
	fn try_push(&mut self, pull_request: PullRequest) {
		pull_request.labels.iter().for_each(|l| {
			for (wl, ps) in WatchedLabels::all().into_iter().zip(self.all_mut().into_iter()) {
				if l.name.as_str() == wl {
					ps.push(pull_request.clone());

					break;
				}
			}
		});
	}

	fn all_mut(&mut self) -> Vec<&mut Vec<PullRequest>> {
		vec![
			&mut self.b3_api,
			&mut self.b5_client,
			&mut self.b7_runtime,
			&mut self.c7_high,
			&mut self.c9_critical,
			&mut self.e1_runtime,
			&mut self.e2_database,
			&mut self.e4_new_host_functions,
			&mut self.e5_breaks_api,
			&mut self.e6_transaction_version,
			&mut self.e7_breaks_authoring,
			&mut self.e8_breaks_everything,
			&mut self.e10_client_update_first,
			&mut self.i8_enhancement,
		]
	}

	/// Make all fields into a [`Vec`].
	pub fn all(self) -> Vec<Vec<PullRequest>> {
		vec![
			self.b3_api,
			self.b5_client,
			self.b7_runtime,
			self.c7_high,
			self.c9_critical,
			self.e1_runtime,
			self.e2_database,
			self.e4_new_host_functions,
			self.e5_breaks_api,
			self.e6_transaction_version,
			self.e7_breaks_authoring,
			self.e8_breaks_everything,
			self.e10_client_update_first,
			self.i8_enhancement,
		]
	}
}
impl From<Vec<PullRequest>> for WatchedPullRequests {
	fn from(pull_requests: Vec<PullRequest>) -> Self {
		let mut w = WatchedPullRequests::default();

		pull_requests.into_iter().for_each(|p| w.try_push(p));

		w
	}
}

/// Track the updates.
///
/// Basically, it compares two commits and return the associated pull requests.
pub async fn track_updates(owner: &str, repo: &str, basehead: &str) -> Result<Vec<PullRequest>> {
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

	let mut pull_requests = stream::iter(commit_shas)
		.enumerate()
		.map(|(i, commit_sha)| {
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
		})
		// TODO: configurable
		.buffer_unordered(64)
		.collect::<Vec<_>>()
		.await;

	pull_requests.sort_by_key(|(i, _)| *i);

	Ok(pull_requests.into_iter().flat_map(|(_, pull_request)| pull_request).collect())
}
