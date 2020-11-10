// --- crates.io ---
use async_std::{sync::Arc, task::block_on};
use clap::ArgMatches;
use futures::{stream, StreamExt};
use isahc::{Body as IsahcBody, ResponseExt};
use serde::de::DeserializeOwned;
// --- githubman ---
use githubman::{
	pager::Pager,
	requests::{
		commits::{
			get_a_commit::GetACommitBuilder, list_commits::ListCommitsBuilder,
			list_pull_requests_associated_with_a_commit::ListPullRequestsAssociatedWithACommitBuilder,
		},
		issues::create_an_issue::CreateAnIssueBuilder,
		releases::list_releases::ListReleasesBuilder,
		repositories::list_repository_tags::ListRepositoryTagsBuilder,
	},
	responses::{
		commits::{Commit, PullRequest},
		releases::Release,
		tags::Tag,
		User,
	},
	GithubApi, Githubman,
};
// --- subalfred ---
use crate::{config::CONFIG, Result};

#[derive(Debug)]
pub struct Substrate {
	pub githubman: Arc<Githubman>,
}
impl Substrate {
	pub const OWNER: &'static str = "paritytech";
	pub const REPO: &'static str = "substrate";

	pub async fn list_repository_tags(&self) -> Result<()> {
		let mut tags = vec![];

		iterate_page_with(
			&self.githubman,
			ListRepositoryTagsBuilder::default()
				.owner(Self::OWNER)
				.repo(Self::REPO)
				.build()
				.unwrap(),
			|mut tags_: Vec<Tag>| tags.append(&mut tags_),
		)
		.await?;

		#[cfg(feature = "dbg")]
		dbg!(tags);

		Ok(())
	}

	pub async fn list_releases(&self) -> Result<()> {
		let mut releases = vec![];

		iterate_page_with(
			&self.githubman,
			ListReleasesBuilder::default()
				.owner(Self::OWNER)
				.repo(Self::REPO)
				.build()
				.unwrap(),
			|mut releases_: Vec<Release>| releases.append(&mut releases_),
		)
		.await?;

		#[cfg(feature = "dbg")]
		dbg!(releases);

		Ok(())
	}

	pub async fn list_commits(&self, list_commits_args: &ArgMatches) -> Result<Vec<Commit>> {
		let mut commits = vec![];
		let date_or_hash = |date_or_hash: &str| -> Result<_> {
			if date_or_hash.contains('-') {
				Ok(date_or_hash.into())
			} else {
				let commit: Commit = block_on(
					self.githubman.send(
						GetACommitBuilder::default()
							.owner(Self::OWNER)
							.repo(Self::REPO)
							.r#ref(date_or_hash)
							.build()
							.unwrap(),
					),
				)?
				.json()?;

				Ok(commit.commit.committer.date)
			}
		};
		let since = if let Some(since) = list_commits_args.value_of("since") {
			Some(date_or_hash(since)?)
		} else {
			None
		};
		let until = if let Some(until) = list_commits_args.value_of("until") {
			Some(date_or_hash(until)?)
		} else {
			None
		};

		iterate_page_with(
			&self.githubman,
			ListCommitsBuilder::default()
				.owner(Self::OWNER)
				.repo(Self::REPO)
				.sha(list_commits_args.value_of("sha").map(Into::into))
				.path(list_commits_args.value_of("path").map(Into::into))
				.since(since)
				.until(until)
				.build()
				.unwrap(),
			|mut commits_: Vec<Commit>| commits.append(&mut commits_),
		)
		.await?;

		#[cfg(feature = "dbg")]
		dbg!(&commits);

		Ok(commits)
	}

	pub async fn list_pull_requests(
		&self,
		list_pull_requests_args: &ArgMatches,
	) -> Result<Vec<PullRequest>> {
		let commit_shas = self
			.list_commits(list_pull_requests_args)
			.await?
			.into_iter()
			.map(|Commit { sha, .. }| sha)
			.collect::<Vec<_>>();
		let mut pull_requests = stream::iter(commit_shas.into_iter().map(|commit_sha| {
			let githubman = self.githubman.clone();
			let request = ListPullRequestsAssociatedWithACommitBuilder::default()
				.owner(Self::OWNER)
				.repo(Self::REPO)
				.commit_sha(commit_sha)
				.build()
				.unwrap();

			async move {
				loop {
					match githubman.send(request.clone()).await {
						Ok(mut response) => match response.json::<Vec<PullRequest>>() {
							Ok(pull_requests) => return pull_requests,
							Err(e) => eprintln!("Serialize Failed Due To: `{:?}`", e),
						},
						Err(e) => eprintln!("Request Failed Due To: `{:?}`", e),
					}
				}
			}
		}))
		.buffer_unordered(
			list_pull_requests_args
				.value_of("thread")
				.map(str::parse)
				.unwrap_or(Ok(1))
				.unwrap(),
		)
		.collect::<Vec<_>>()
		.await
		.into_iter()
		.flatten()
		.collect::<Vec<_>>();

		pull_requests.sort_by(|a, b| b.merged_at.cmp(&a.merged_at));

		#[cfg(feature = "dbg")]
		dbg!(&pull_requests);

		if list_pull_requests_args.is_present("create-issue") {
			let mut body = String::new();

			for PullRequest {
				html_url,
				title,
				user: User {
					login,
					html_url: user_html_url,
				},
				body: pull_request_body,
				merged_at,
				labels,
				..
			} in &pull_requests
			{
				let migration = if labels
					.iter()
					.any(|label| &label.name == "D1-runtime-migration")
				{
					" - !!Contains Migration!!"
				} else {
					""
				};
				let pull_request_body = pull_request_body.replace('\n', "\n\t  ");
				let pull_request_body = pull_request_body.trim_end();

				body.push_str(&format!(
					"- [ ] [{}]({})\n\
					\t- *by [{}]({}) merged at {}*\n\
					\t- <details>\n\
					\t  <summary>Details{}</summary>\n\
					\t  {}\n\
					\t  </details>\n",
					title, html_url, login, user_html_url, merged_at, migration, pull_request_body
				));
			}

			create_an_issue(
				&self.githubman,
				&CONFIG.substrate_project.owner,
				&CONFIG.substrate_project.issue_repo,
				"Updates",
				body,
			)
			.await?;
		}

		Ok(pull_requests)
	}

	pub async fn list_migrations(
		&self,
		list_migrations_args: &ArgMatches,
	) -> Result<Vec<PullRequest>> {
		let mut pull_requests = self.list_pull_requests(list_migrations_args).await?;
		pull_requests.retain(|pull_request| {
			pull_request
				.labels
				.iter()
				.any(|label| &label.name == "D1-runtime-migration")
		});

		#[cfg(feature = "dbg")]
		dbg!(&pull_requests);

		if list_migrations_args.is_present("create-issue") {
			let mut body = String::new();

			for PullRequest {
				html_url,
				title,
				user: User {
					login,
					html_url: user_html_url,
				},
				body: pull_request_body,
				merged_at,
				..
			} in &pull_requests
			{
				let pull_request_body = pull_request_body.replace('\n', "\n\t  ");
				let pull_request_body = pull_request_body.trim_end();

				body.push_str(&format!(
					"- [ ] [{}]({})\n\
					\t- *by [{}]({}) merged at {}*\n\
					\t- <details>\n\
					\t  <summary>Details</summary>\n\
					\t  {}\n\
					\t  </details>\n",
					title, html_url, login, user_html_url, merged_at, pull_request_body
				));
			}

			create_an_issue(
				&self.githubman,
				&CONFIG.substrate_project.owner,
				&CONFIG.substrate_project.issue_repo,
				"Migrations",
				body,
			)
			.await?;
		}

		Ok(pull_requests)
	}
}

async fn iterate_page_with<B, D, F>(
	githubman: &Arc<Githubman>,
	request: impl GithubApi<B>,
	mut f: F,
) -> Result<()>
where
	B: Into<IsahcBody>,
	D: DeserializeOwned,
	F: FnMut(Vec<D>),
{
	let mut pager = Pager {
		per_page: 100,
		page: 1,
	};

	loop {
		let ds: Vec<D> = githubman
			.send_with_pager(request.clone(), &mut pager)
			.await?
			.json()?;

		if ds.is_empty() {
			return Ok(());
		}

		f(ds);
	}
}

async fn create_an_issue(
	githubman: &Arc<Githubman>,
	owner: impl Into<String>,
	repo: impl Into<String>,
	title: impl Into<String>,
	body: impl Into<String>,
) -> Result<()> {
	githubman
		.send(
			CreateAnIssueBuilder::default()
				.owner(owner.into())
				.repo(repo.into())
				.title(format!(
					"{} - Automatically Generated By Subalfred",
					title.into()
				))
				.body(Some(body.into()))
				.build()
				.unwrap(),
		)
		.await?;

	Ok(())
}
