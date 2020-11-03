#![feature(with_options)]

pub mod config;
pub mod substrate;

// --- std ---
use std::fs::{create_dir_all, File};
// --- crates.io ---
use app_dirs2::*;
use async_std::{sync::Arc, task};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githubman::{
	pager::Pager,
	requests::commits::{
		list_commits::ListCommitsBuilder,
		list_pull_requests_associated_with_a_commit::ListPullRequestsAssociatedWithACommitBuilder,
	},
	responses::commits::{Commit, PullRequest},
	GithubMan,
};
use isahc::ResponseExt;
// --- subalfred ---
use config::Config;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const APP_INFO: AppInfo = AppInfo {
	name: crate_name!(),
	author: crate_authors!(),
};

#[async_std::main]
pub async fn main() -> Result<()> {
	let app_root_path = get_app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
	let app_config_path = app_root_path.join("config");
	let file = if app_config_path.is_file() {
		File::with_options()
			.create(false)
			.read(true)
			.write(true)
			.append(false)
			.open(&app_config_path)
			.unwrap()
	} else {
		if !app_root_path.is_dir() {
			create_dir_all(&app_root_path).unwrap();
		}

		let mut file = File::with_options();

		file.create_new(true).read(true).write(true).append(false);

		#[cfg(target_family = "unix")]
		{
			// --- std ---
			use std::os::unix::fs::OpenOptionsExt;

			file.mode(0o600);
		}

		file.open(&app_config_path).unwrap()
	};
	let config = if let Ok(config) = Config::from_reader(&file) {
		config
	} else {
		let config = Config::default();

		config.to_writer(&file).unwrap();

		config
	};
	let app = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.subcommand(
			App::new("list-commits").about("").arg(
				Arg::new("sha")
					.long("sha")
					.takes_value(true)
					.value_name("BRANCH/HASH")
					.about(
						"SHA or branch to start listing commits from. \
						Default: the repository’s default branch (usually master).",
					),
			),
		)
		.subcommand(
			App::new("list-migrations")
				.about("")
				.arg(
					Arg::new("thread")
						.long("thread")
						.takes_value(true)
						.value_name("COUNT")
						.about(""),
				)
				.arg(
					Arg::new("sha")
						.long("sha")
						.value_name("BRANCH/HASH")
						.takes_value(true),
				)
				.about(
					"SHA or branch to start listing commits from.\
					Default: the repository’s default branch (usually master).",
				)
				.arg(
					Arg::new("path")
						.long("path")
						.takes_value(true)
						.value_name("PATH")
						.about("Only commits containing this file path will be returned."),
				)
				.arg(
					Arg::new("since")
						.long("since")
						.takes_value(true)
						.value_name("DATE")
						.about(
							"Only show notifications updated after the given time. \
							This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.",
						),
				)
				.arg(
					Arg::new("until")
						.long("until")
						.takes_value(true)
						.value_name("DATE")
						.about(
							"Only commits before this date will be returned. \
							This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.",
						),
				),
		);
	let app_args = app.get_matches();
	let github_man = GithubMan::new(&config.github_oauth_token);

	if let Some(list_commits_args) = app_args.subcommand_matches("list-commits") {
		let json: serde_json::Value = github_man
			.get(
				ListCommitsBuilder::default()
					.owner("paritytech")
					.repo("substrate")
					.sha(list_commits_args.value_of("sha").map(Into::into))
					.per_page(Some(100u32))
					.build()
					.unwrap(),
			)
			.await?
			.json()?;

		#[cfg(feature = "dbg")]
		dbg!(json);
	} else if let Some(list_migrations_args) = app_args.subcommand_matches("list-migrations") {
		let mut commit_shas = vec![];
		let mut pager = Pager {
			per_page: 100,
			page: 1,
		};

		loop {
			let commits: Vec<Commit> = github_man
				.get_with_pager(
					ListCommitsBuilder::default()
						.owner("paritytech")
						.repo("substrate")
						.sha(list_migrations_args.value_of("sha").map(Into::into))
						.path(list_migrations_args.value_of("path").map(Into::into))
						.since(list_migrations_args.value_of("since").map(Into::into))
						.until(list_migrations_args.value_of("until").map(Into::into))
						.build()
						.unwrap(),
					&mut pager,
				)
				.await?
				.json()?;

			if commits.is_empty() {
				break;
			}

			for Commit { sha, .. } in commits {
				commit_shas.push(sha);
			}
		}

		#[cfg(feature = "dbg")]
		dbg!(&commit_shas);

		let github_man = Arc::new(github_man);
		let mut migrations = vec![];

		for chunk in commit_shas.chunks(32) {
			let mut handles = vec![];

			for commit_sha in chunk.iter() {
				let github_man = github_man.clone();
				let req = ListPullRequestsAssociatedWithACommitBuilder::default()
					.owner("paritytech")
					.repo("substrate")
					.commit_sha(commit_sha)
					.build()
					.unwrap();
				let commit_sha = commit_sha.to_owned();

				handles.push(task::spawn(async move {
					(github_man.get(req).await, commit_sha)
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
	}

	Ok(())
}
