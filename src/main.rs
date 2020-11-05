#![feature(with_options)]

pub mod config;
pub mod node;
pub mod substrate;

// --- crates.io ---
use app_dirs2::AppInfo;
use async_std::sync::Arc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githubman::Githubman;
// --- subalfred ---
use config::CONFIG;
use substrate::Substrate;

type Error = Box<dyn std::error::Error>;
type Result<T> = ::std::result::Result<T, Error>;

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
		.subcommand(
			App::new("send-rpc")
				.about("")
				.arg(
					Arg::new("address")
						.long("address")
						.takes_value(true)
						.value_name("ADDRESS")
						.about(""),
				)
				.arg(
					Arg::new("method")
						.long("method")
						.required(true)
						.takes_value(true)
						.value_name("METHOD")
						.possible_values(&["author_rotateKeys", "author_insertKey"])
						.about(""),
				)
				.arg(
					Arg::new("params")
						.long("params")
						.takes_value(true)
						.value_name("[PARAM]")
						.about(""),
				),
		)
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
			App::new("list-pull-requests")
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
	let githubman = Githubman::new(&CONFIG.github_oauth_token);
	let substrate = Substrate {
		githubman: Arc::new(githubman),
	};

	if let Some(send_rpc) = app_args.subcommand_matches("send-rpc") {
		node::send_rpc(
			send_rpc
				.value_of("address")
				.unwrap_or("http://127.0.0.1:9933"),
			send_rpc.value_of("method").unwrap(),
			serde_json::from_str(send_rpc.value_of("params").unwrap_or("[]"))?,
		)
		.await?;
	} else if let Some(_) = app_args.subcommand_matches("list-repository-tags") {
		substrate.list_repository_tags().await?;
	} else if let Some(_) = app_args.subcommand_matches("list-releases") {
		substrate.list_releases().await?;
	} else if let Some(list_commits_args) = app_args.subcommand_matches("list-commits") {
		substrate.list_commits(list_commits_args).await?;
	} else if let Some(list_pull_requests_args) = app_args.subcommand_matches("list-pull-requests")
	{
		substrate
			.list_pull_requests(list_pull_requests_args)
			.await?;
	} else if let Some(list_migrations_args) = app_args.subcommand_matches("list-migrations") {
		substrate.list_migrations(list_migrations_args).await?;
	}

	Ok(())
}
