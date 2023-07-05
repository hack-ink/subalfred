//! Collection of Substrate GitHub related functions.

// crates.io
use futures::{stream, StreamExt};
use githuber::api::commits;
// subalfred
use super::*;
use crate::prelude::*;

/// The specific labels that worth to watch.
pub enum WatchedLabels {
	/// [https://github.com/paritytech/substrate/labels/C5-high]
	C5High,
	/// [https://github.com/paritytech/substrate/labels/C7-critical]
	C7Critical,
	/// [https://github.com/paritytech/substrate/labels/D2-breaksapi]
	D2BreaksApi,
	/// [https://github.com/paritytech/substrate/labels/E0-runtime_migration]
	E0RuntimeMigration,
	/// [https://github.com/paritytech/substrate/labels/E1-database_migration]
	E1DatabaseMigration,
	/// [https://github.com/paritytech/substrate/labels/E3-host_functions]
	E3HostFunctions,
	/// [https://github.com/paritytech/substrate/labels/E4-node_first_update]
	E4NodeFirstUpdate,
	/// [https://github.com/paritytech/substrate/labels/F0-breaks_everything]
	F0BreaksEverything,
	/// [https://github.com/paritytech/substrate/labels/F1-breaks_authoring]
	F1BreaksAuthoring,
	/// [https://github.com/paritytech/substrate/labels/F2-breaks_consensus]
	F2BreaksConsensus,
	/// [https://github.com/paritytech/substrate/labels/F3-breaks_API]
	F3BreaksApi,
}
impl WatchedLabels {
	const fn as_str(&self) -> &'static str {
		// subalfred
		use WatchedLabels::*;

		match self {
			C5High => "C5-high",
			C7Critical => "C7-critical",
			D2BreaksApi => "D2-breaksapi",
			E0RuntimeMigration => "E0-runtime_migration",
			E1DatabaseMigration => "E1-database_migration",
			E3HostFunctions => "E3-host_functions",
			E4NodeFirstUpdate => "E4-node_first_update",
			F0BreaksEverything => "F0-breaks_everything",
			F1BreaksAuthoring => "F1-breaks_authoring",
			F2BreaksConsensus => "F2-breaks_consensus",
			F3BreaksApi => "F3-breaks_API",
		}
	}

	/// Return all labels' name.
	pub fn all() -> Vec<&'static str> {
		// subalfred
		use WatchedLabels::*;

		vec![
			C5High.as_str(),
			C7Critical.as_str(),
			D2BreaksApi.as_str(),
			E0RuntimeMigration.as_str(),
			E1DatabaseMigration.as_str(),
			E3HostFunctions.as_str(),
			E4NodeFirstUpdate.as_str(),
			F0BreaksEverything.as_str(),
			F1BreaksAuthoring.as_str(),
			F2BreaksConsensus.as_str(),
			F3BreaksApi.as_str(),
		]
	}
}

/// The pull requests with specific labels that worth to watch.
#[derive(Debug, Default)]
pub struct WatchedPullRequests {
	c5_high: Vec<PullRequest>,
	c7_critical: Vec<PullRequest>,
	d2_breaks_api: Vec<PullRequest>,
	e0_runtime_migration: Vec<PullRequest>,
	e1_database_migration: Vec<PullRequest>,
	e3_host_functions: Vec<PullRequest>,
	e4_node_first_update: Vec<PullRequest>,
	f0_breaks_everything: Vec<PullRequest>,
	f1_breaks_authoring: Vec<PullRequest>,
	f2_breaks_consensus: Vec<PullRequest>,
	f3_breaks_api: Vec<PullRequest>,
}
impl WatchedPullRequests {
	fn try_push(&mut self, pull_request: PullRequest) {
		pull_request.labels.iter().for_each(|l| {
			for (wl, ps) in WatchedLabels::all().into_iter().zip(self.all_mut()) {
				if l.name.as_str() == wl {
					ps.push(pull_request.clone());

					break;
				}
			}
		});
	}

	fn all_mut(&mut self) -> Vec<&mut Vec<PullRequest>> {
		vec![
			&mut self.c5_high,
			&mut self.c7_critical,
			&mut self.d2_breaks_api,
			&mut self.e0_runtime_migration,
			&mut self.e1_database_migration,
			&mut self.e3_host_functions,
			&mut self.e4_node_first_update,
			&mut self.f0_breaks_everything,
			&mut self.f1_breaks_authoring,
			&mut self.f2_breaks_consensus,
			&mut self.f3_breaks_api,
		]
	}

	/// Make all fields into a [`Vec`].
	pub fn all(self) -> Vec<Vec<PullRequest>> {
		vec![
			self.c5_high,
			self.c7_critical,
			self.d2_breaks_api,
			self.e0_runtime_migration,
			self.e1_database_migration,
			self.e3_host_functions,
			self.e4_node_first_update,
			self.f0_breaks_everything,
			self.f1_breaks_authoring,
			self.f2_breaks_consensus,
			self.f3_breaks_api,
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
