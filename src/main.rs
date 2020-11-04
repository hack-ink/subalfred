#![feature(with_options)]

pub mod config;
pub mod substrate;

// --- crates.io ---
use app_dirs2::AppInfo;
use async_std::sync::Arc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githubman::Githubman;
// --- subalfred ---
use config::Config;
use substrate::Substrate;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const APP_INFO: AppInfo = AppInfo {
	name: crate_name!(),
	author: crate_authors!(),
};

#[async_std::main]
pub async fn main() -> Result<()> {
	let app = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.subcommand(App::new("list-repository-tags").about(""))
		.subcommand(App::new("list-releases").about(""))
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
				)
				.arg(Arg::new("create-issue").long("create-issue")),
		);
	let app_args = app.get_matches();
	let config = Config::load_config();
	let githubman = Githubman::new(&config.github_oauth_token);
	let substrate = Substrate {
		githubman: Arc::new(githubman),
	};

	if let Some(_) = app_args.subcommand_matches("list-repository-tags") {
		substrate.list_repository_tags().await?;
	} else if let Some(_) = app_args.subcommand_matches("list-releases") {
		substrate.list_releases().await?;
	} else if let Some(list_commits_args) = app_args.subcommand_matches("list-commits") {
		substrate.list_commits(list_commits_args).await?;
	} else if let Some(list_migrations_args) = app_args.subcommand_matches("list-migrations") {
		let maybe_self_project = if list_migrations_args.is_present("create-issue") {
			Some((
				config.substrate_project_owner.as_str(),
				config.substrate_project_repo.as_str(),
			))
		} else {
			None
		};

		substrate
			.list_migrations(list_migrations_args, maybe_self_project)
			.await?;
	}

	Ok(())
}
