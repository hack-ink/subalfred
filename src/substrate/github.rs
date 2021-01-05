// --- crates.io ---
use async_std::{sync::Arc, task};
use futures::{stream, StreamExt};
use isahc::{AsyncBody as IsahcBody, AsyncReadResponseExt};
use serde::de::DeserializeOwned;
// --- githuber ---
use githuber::{
	pager::Pager,
	requests::{
		commits::{
			get_a_commit::GetACommitBuilder, list_commits::ListCommitsBuilder,
			list_pull_requests_associated_with_a_commit::ListPullRequestsAssociatedWithACommitBuilder,
		},
		contents::get_repository_content::GetRepositoryContentBuilder,
		issues::create_an_issue::CreateAnIssueBuilder,
		releases::list_releases::ListReleasesBuilder,
		repositories::list_repository_tags::ListRepositoryTagsBuilder,
	},
	responses::{
		commits::{Commit, PullRequest, User},
		contents::Content,
		releases::Release,
		repositories::Tag,
	},
	GithubApi, Githuber,
};
use tracing::trace;
// --- subalfred ---
use crate::{AnyResult, Subalfred};

impl Subalfred {
	pub const SUBSTRATE_GITHUB_OWNER: &'static str = "paritytech";
	pub const SUBSTRATE_GITHUB_REPO: &'static str = "substrate";

	pub async fn list_tags(&self) -> AnyResult<Vec<Tag>> {
		let mut tags = vec![];

		iterate_page_with(
			&self.githuber,
			ListRepositoryTagsBuilder::default()
				.owner(Self::SUBSTRATE_GITHUB_OWNER)
				.repo(Self::SUBSTRATE_GITHUB_REPO)
				.build()
				.unwrap(),
			|mut tags_: Vec<Tag>| tags.append(&mut tags_),
		)
		.await?;

		trace!("{:#?}", tags);

		Ok(tags)
	}

	pub async fn list_releases(&self) -> AnyResult<Vec<Release>> {
		let mut releases = vec![];

		iterate_page_with(
			&self.githuber,
			ListReleasesBuilder::default()
				.owner(Self::SUBSTRATE_GITHUB_OWNER)
				.repo(Self::SUBSTRATE_GITHUB_REPO)
				.build()
				.unwrap(),
			|mut releases_: Vec<Release>| releases.append(&mut releases_),
		)
		.await?;

		trace!("{:#?}", releases);

		Ok(releases)
	}

	pub async fn list_commits(
		&self,
		sha: Option<&str>,
		path: Option<&str>,
		since: Option<&str>,
		until: Option<&str>,
	) -> AnyResult<Vec<Commit>> {
		let mut commits = vec![];
		let date_or_hash = |date_or_hash: &str| -> AnyResult<_> {
			if date_or_hash.contains('-') {
				Ok(date_or_hash.into())
			} else {
				let mut v = vec![];
				let mut response = task::block_on(
					self.githuber.send(
						GetACommitBuilder::default()
							.owner(Self::SUBSTRATE_GITHUB_OWNER)
							.repo(Self::SUBSTRATE_GITHUB_REPO)
							.r#ref(date_or_hash)
							.build()
							.unwrap(),
					),
				)?;
				task::block_on(response.copy_to(&mut v))?;
				let commit: Commit = serde_json::from_slice(&v)?;

				Ok(commit.commit.committer.date)
			}
		};
		let since = if let Some(since) = since {
			Some(date_or_hash(since)?)
		} else {
			None
		};
		let until = if let Some(until) = until {
			Some(date_or_hash(until)?)
		} else {
			None
		};

		iterate_page_with(
			&self.githuber,
			ListCommitsBuilder::default()
				.owner(Self::SUBSTRATE_GITHUB_OWNER)
				.repo(Self::SUBSTRATE_GITHUB_REPO)
				.sha(sha.map(Into::into))
				.path(path.map(Into::into))
				.since(since)
				.until(until)
				.build()
				.unwrap(),
			|mut commits_: Vec<Commit>| commits.append(&mut commits_),
		)
		.await?;

		trace!("{:#?}", commits);

		Ok(commits)
	}

	pub async fn list_pull_requests(
		&self,
		sha: Option<&str>,
		path: Option<&str>,
		since: Option<&str>,
		until: Option<&str>,
		thread: usize,
		create_issue: bool,
	) -> AnyResult<Vec<PullRequest>> {
		let commits = self.list_commits(sha, path, since, until).await?;
		let mut pull_requests = stream::iter(commits.into_iter().map(|Commit { sha, .. }| {
			let githuber = self.githuber.clone();
			let request = ListPullRequestsAssociatedWithACommitBuilder::default()
				.owner(Self::SUBSTRATE_GITHUB_OWNER)
				.repo(Self::SUBSTRATE_GITHUB_REPO)
				.commit_sha(sha)
				.build()
				.unwrap();

			async move {
				loop {
					match githuber.send(request.clone()).await {
						Ok(mut response) => {
							let mut v = vec![];

							if let Err(e) = response.copy_to(&mut v).await {
								eprintln!("Read Response Body Failed Due To: `{:?}`", e);
							} else {
								match serde_json::from_slice::<Vec<PullRequest>>(&v) {
									Ok(pull_requests) => return pull_requests,
									Err(e) => eprintln!("Serialize Failed Due To: `{:?}`", e),
								}
							}
						}
						Err(e) => eprintln!("Request Failed Due To: `{:?}`", e),
					}
				}
			}
		}))
		.buffer_unordered(thread)
		.collect::<Vec<_>>()
		.await
		.into_iter()
		.flatten()
		.collect::<Vec<_>>();

		pull_requests.sort_by(|a, b| b.merged_at.cmp(&a.merged_at));

		trace!("{:#?}", pull_requests);

		if create_issue {
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

			self.create_an_issue(
				&self.project.owner,
				&self.project.issue_repo,
				"Updates",
				body,
			)
			.await?;
		}

		Ok(pull_requests)
	}

	pub async fn list_migrations(
		&self,
		sha: Option<&str>,
		path: Option<&str>,
		since: Option<&str>,
		until: Option<&str>,
		thread: usize,
		create_issue: bool,
	) -> AnyResult<Vec<PullRequest>> {
		let mut pull_requests = self
			.list_pull_requests(sha, path, since, until, thread, false)
			.await?;
		pull_requests.retain(|pull_request| {
			pull_request
				.labels
				.iter()
				.any(|label| &label.name == "D1-runtime-migration")
		});

		trace!("{:#?}", pull_requests);

		if create_issue {
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

			self.create_an_issue(
				&self.project.owner,
				&self.project.issue_repo,
				"Migrations",
				body,
			)
			.await?;
		}

		Ok(pull_requests)
	}

	pub async fn get_repository_content(
		&self,
		owner: impl Into<String>,
		repo: impl Into<String>,
		path: impl Into<String>,
		r#ref: Option<&str>,
	) -> AnyResult<Content> {
		let mut v = vec![];
		self.githuber
			.send(
				GetRepositoryContentBuilder::default()
					.owner(owner)
					.repo(repo)
					.path(path)
					.r#ref(r#ref.map(Into::into))
					.build()
					.unwrap(),
			)
			.await?
			.copy_to(&mut v)
			.await?;
		let content = serde_json::from_slice(&v)?;

		trace!("{:#?}", content);

		Ok(content)
	}

	pub async fn create_an_issue(
		&self,
		owner: impl Into<String>,
		repo: impl Into<String>,
		title: impl Into<String>,
		body: impl Into<String>,
	) -> AnyResult<()> {
		self.githuber
			.send(
				CreateAnIssueBuilder::default()
					.owner(owner)
					.repo(repo)
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
}

async fn iterate_page_with<B, D, F>(
	githuber: &Arc<Githuber>,
	request: impl GithubApi<B>,
	mut f: F,
) -> AnyResult<()>
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
		let mut v = vec![];
		githuber
			.send_with_pager(request.clone(), &mut pager)
			.await?
			.copy_to(&mut v)
			.await?;
		let ds = serde_json::from_slice::<Vec<D>>(&v)?;

		if ds.is_empty() {
			return Ok(());
		}

		f(ds);
	}
}
