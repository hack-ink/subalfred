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
		releases::list_releases::ListReleasesBuilder,
		repositories::list_repository_tags::ListRepositoryTagsBuilder,
	},
	responses::commits::{Commit, PullRequest},
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
		let json: serde_json::Value = self
			.githubman
			.send(
				ListRepositoryTagsBuilder::default()
					.owner(Self::OWNER)
					.repo(Self::REPO)
					.per_page(Some(100u32))
					.build()
					.unwrap(),
			)
			.await?
			.json()?;

		#[cfg(feature = "dbg")]
		dbg!(json);

		Ok(())
	}

	pub async fn list_releases(&self) -> Result<()> {
		let json: serde_json::Value = self
			.githubman
			.send(
				ListReleasesBuilder::default()
					.owner(Self::OWNER)
					.repo(Self::REPO)
					.per_page(Some(100u32))
					.build()
					.unwrap(),
			)
			.await?
			.json()?;

		#[cfg(feature = "dbg")]
		dbg!(json);

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

	pub async fn list_migrations(&self, list_migrations_args: &ArgMatches) -> Result<()> {
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

		// let githubman = Arc::new(self.githubman);
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
				let commit_sha = commit_sha.to_owned();

				handles.push(task::spawn(async move {
					(githubman.send(request).await, commit_sha)
				}));
			}

			for handle in handles {
				let (pull_requests, commit_sha) = handle.await;
				let pull_requests: Vec<PullRequest> = pull_requests?.json()?;

				for PullRequest { url, labels, .. } in pull_requests {
					if labels
						.into_iter()
						.any(|label| &label.name == "D1-runtime-migration")
					{
						migrations.push((url, commit_sha.clone()));
					}
				}
			}
		}

		#[cfg(feature = "dbg")]
		dbg!(&migrations);

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
