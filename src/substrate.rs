// --- crates.io ---
use async_std::{sync::Arc, task};
use clap::ArgMatches;
use isahc::{Body as IsahcBody, ResponseExt};
use serde::de::DeserializeOwned;
// --- githubman ---
use githubman::{
	pager::Pager,
	requests::{
		commits::{
			list_commits::ListCommitsBuilder,
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
use crate::Result;

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

	pub async fn list_commits(&self, list_commits_args: &ArgMatches) -> Result<()> {
		let mut commit_shas = vec![];

		iterate_page_with(
			&self.githubman,
			ListCommitsBuilder::default()
				.owner(Self::OWNER)
				.repo(Self::REPO)
				.sha(list_commits_args.value_of("sha").map(Into::into))
				.path(list_commits_args.value_of("path").map(Into::into))
				.since(list_commits_args.value_of("since").map(Into::into))
				.until(list_commits_args.value_of("until").map(Into::into))
				.build()
				.unwrap(),
			|commits: Vec<Commit>| {
				for Commit { sha, .. } in commits {
					commit_shas.push(sha);
				}
			},
		)
		.await?;

		#[cfg(feature = "dbg")]
		dbg!(&commit_shas);

		Ok(())
	}

	pub async fn list_migrations(
		&self,
		list_migrations_args: &ArgMatches,
		maybe_self_project: Option<(&str, &str)>,
	) -> Result<()> {
		let mut commit_shas = vec![];

		iterate_page_with(
			&self.githubman,
			ListCommitsBuilder::default()
				.owner(Self::OWNER)
				.repo(Self::REPO)
				.sha(list_migrations_args.value_of("sha").map(Into::into))
				.path(list_migrations_args.value_of("path").map(Into::into))
				.since(list_migrations_args.value_of("since").map(Into::into))
				.until(list_migrations_args.value_of("until").map(Into::into))
				.build()
				.unwrap(),
			|commits: Vec<Commit>| {
				for Commit { sha, .. } in commits {
					commit_shas.push(sha);
				}
			},
		)
		.await?;

		#[cfg(feature = "dbg")]
		dbg!(&commit_shas);

		let mut migrations = vec![];

		for chunk in commit_shas.chunks(
			list_migrations_args
				.value_of("thread")
				.map(str::parse)
				.unwrap_or(Ok(1))
				.unwrap(),
		) {
			let mut handles = vec![];

			for commit_sha in chunk.iter() {
				let githubman = self.githubman.clone();
				let request = ListPullRequestsAssociatedWithACommitBuilder::default()
					.owner(Self::OWNER)
					.repo(Self::REPO)
					.commit_sha(commit_sha)
					.build()
					.unwrap();

				handles.push(task::spawn(async move { githubman.send(request).await }));
			}

			for handle in handles {
				let pull_requests: Vec<PullRequest> = handle.await?.json()?;

				for pull_request in pull_requests {
					if pull_request
						.labels
						.iter()
						.any(|label| &label.name == "D1-runtime-migration")
					{
						migrations.push(pull_request);
					}
				}
			}
		}

		#[cfg(feature = "dbg")]
		dbg!(&migrations);

		if let Some((owner, repo)) = maybe_self_project {
			let mut body = String::new();

			for PullRequest {
				html_url,
				title,
				user: User {
					login,
					html_url: user_html_url,
				},
				merged_at,
				..
			} in migrations
			{
				body.push_str(&format!(
					"- [ ] [**{}**]({})\n\t- by [**{}**]({}) merged at **{}**\n",
					title, html_url, login, user_html_url, merged_at
				));
			}

			self.githubman
				.send(
					CreateAnIssueBuilder::default()
						.owner(owner)
						.repo(repo)
						.title("Migrations")
						.body(Some(body))
						.build()
						.unwrap(),
				)
				.await?;
		}

		Ok(())
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
