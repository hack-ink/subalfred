#![feature(with_options)]

pub mod config;
pub mod node;
pub mod substrate;

// --- std ---
use std::env;
// --- crates.io ---
use app_dirs2::AppInfo;
use async_std::sync::Arc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githubman::Githubman;
// --- subalfred ---
use crate::config::Project;

type Error = Box<dyn std::error::Error>;
type Result<T> = ::std::result::Result<T, Error>;

const APP_INFO: AppInfo = AppInfo {
	name: crate_name!(),
	author: crate_authors!(),
};

#[async_std::main]
async fn main() -> Result<()> {
	let app = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(
			Arg::new("log")
				.long("long")
				.short('l')
				.takes_value(true)
				.value_name("TARGET")
				.global(true)
				.about(""),
		)
		.subcommand(App::new("list-repository-tags").about(""))
		.subcommand(App::new("list-releases").about(""))
		.subcommand(
			App::new("list-commits").about("").arg(
				Arg::new("sha")
					.long("sha")
					.takes_value(true)
					.value_name("BRANCH/SHA")
					.about(
						"SHA or branch to start listing commits from. \
						Default: the repository’s default branch (usually master).",
					),
			),
		)
		.subcommand(list_app("list-pull-requests"))
		.subcommand(list_app("list-migrations"))
		.subcommand(
			App::new("send-rpc")
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
						.about(""),
				)
				.arg(
					Arg::new("params")
						.long("params")
						.takes_value(true)
						.value_name("[PARAM]")
						.about(""),
				)
				.about(""),
		)
		.subcommand(App::new("check-runtime-version").about(""));
	let app_args = app.get_matches();
	let subalfred = Subalfred::init();

	if let Some(logs) = app_args.values_of("log") {
		for log in logs {
			env::set_var("RUST_LOG", log);
		}

		pretty_env_logger::init();
	}

	if let Some(_) = app_args.subcommand_matches("list-repository-tags") {
		subalfred.list_repository_tags().await?;
	} else if let Some(_) = app_args.subcommand_matches("list-releases") {
		subalfred.list_releases().await?;
	} else if let Some(list_commits_args) = app_args.subcommand_matches("list-commits") {
		subalfred.list_commits(list_commits_args).await?;
	} else if let Some(list_pull_requests_args) = app_args.subcommand_matches("list-pull-requests")
	{
		subalfred
			.list_pull_requests(list_pull_requests_args)
			.await?;
	} else if let Some(list_migrations_args) = app_args.subcommand_matches("list-migrations") {
		subalfred.list_migrations(list_migrations_args).await?;
	} else if let Some(send_rpc_args) = app_args.subcommand_matches("send-rpc") {
		Subalfred::send_rpc(
			send_rpc_args
				.value_of("address")
				.unwrap_or("http://127.0.0.1:9933"),
			send_rpc_args.value_of("method").unwrap(),
			serde_json::from_str(send_rpc_args.value_of("params").unwrap_or("[]"))?,
		)
		.await?;
	} else if let Some(_) = app_args.subcommand_matches("check-runtime-version") {
		subalfred.check_runtime_version().await?;
	}

	Ok(())
}

fn list_app(name: &str) -> App {
	App::new(name)
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
				.value_name("BRANCH/SHA")
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
				.value_name("DATE/SHA")
				.about(
					"Only show notifications updated after the given time. \
					This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.",
				),
		)
		.arg(
			Arg::new("until")
				.long("until")
				.takes_value(true)
				.value_name("DATE/SHA")
				.about(
					"Only commits before this date will be returned. \
					This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.",
				),
		)
		.arg(Arg::new("create-issue").long("create-issue"))
}

struct Subalfred {
	githubman: Arc<Githubman>,
	project: Project,
}
