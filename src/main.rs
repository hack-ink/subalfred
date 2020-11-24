#![feature(with_options)]

pub mod config;
pub mod substrate;

// --- std ---
use std::{env, process::Command};
// --- crates.io ---
use anyhow::Result as AnyResult;
use app_dirs2::AppInfo;
use async_std::sync::Arc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githuber::Githuber;
use isahc::ResponseExt;
use serde_json::Value;
// --- subalfred ---
use crate::{
	config::Project,
	substrate::{
		crypto::parse_account,
		hash::{hash, parse_storage_keys},
	},
};

const APP_INFO: AppInfo = AppInfo {
	name: crate_name!(),
	author: crate_authors!(),
};

#[async_std::main]
async fn main() -> AnyResult<()> {
	// TODO: about
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
		.subcommand(list_app("list-commits").about(""))
		.subcommand(
			list_app("list-pull-requests")
				.about("")
				.arg(
					Arg::new("thread")
						.long("thread")
						.takes_value(true)
						.value_name("COUNT")
						.about(""),
				)
				.arg(Arg::new("create-issue").long("create-issue")),
		)
		.subcommand(
			list_app("list-migrations")
				.about("")
				.arg(
					Arg::new("thread")
						.long("thread")
						.takes_value(true)
						.value_name("COUNT")
						.about(""),
				)
				.arg(Arg::new("create-issue").long("create-issue")),
		)
		.subcommand(
			App::new("send-rpc")
				.about("")
				.arg(
					Arg::new("uri")
						.long("uri")
						.takes_value(true)
						.value_name("URI")
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
				),
		)
		.subcommand(App::new("check-runtime-version").about(""))
		.subcommand(
			App::new("account").about("").arg(
				Arg::new("account")
					.required(true)
					.takes_value(true)
					.value_name("PUBLIC KEY/SS58 ADDRESS")
					.about(""),
			),
		)
		.subcommand(
			App::new("hash")
				.about("")
				.arg(
					Arg::new("data")
						.required(true)
						.takes_value(true)
						.value_name("VALUE")
						.about(""),
				)
				.arg(
					Arg::new("hasher")
						.long("hasher")
						.takes_value(true)
						.possible_values(&[
							"blake2-128",
							"blake2-256",
							"blake2-128-concat",
							"twox-64",
							"twox-128",
							"twox-256",
							"twox-64-concat",
							"identity",
						])
						.value_name("HASHER")
						.about(""),
				)
				.arg(Arg::new("hex").long("hex").about("")),
		)
		.subcommand(
			// TODO: handle instance
			App::new("storage-prefix")
				.about("")
				.arg(
					Arg::new("prefix")
						.long("prefix")
						.takes_value(true)
						.value_name("NAME")
						.about(""),
				)
				.arg(
					Arg::new("item")
						.long("item")
						.takes_value(true)
						.value_name("NAME")
						.about(""),
				),
		)
		.subcommand(
			App::new("node-template").about("").arg(
				Arg::new("name")
					.long("name")
					.takes_value(true)
					.value_name("NAME")
					.about(""),
			),
		);
	let app_args = app.get_matches();

	if let Some(logs) = app_args.values_of("log") {
		for log in logs {
			env::set_var("RUST_LOG", log);
		}

		pretty_env_logger::init_timed();
	}

	let subalfred = Subalfred::init();

	// TODO: beautify output
	if let Some(_) = app_args.subcommand_matches("list-repository-tags") {
		println!("{:#?}", subalfred.list_repository_tags().await?);
	} else if let Some(_) = app_args.subcommand_matches("list-releases") {
		println!("{:#?}", subalfred.list_releases().await?);
	} else if let Some(list_commits_args) = app_args.subcommand_matches("list-commits") {
		println!(
			"{:#?}",
			subalfred
				.list_commits(
					list_commits_args.value_of("sha"),
					list_commits_args.value_of("path"),
					list_commits_args.value_of("since"),
					list_commits_args.value_of("until"),
				)
				.await?
		);
	} else if let Some(list_pull_requests_args) = app_args.subcommand_matches("list-pull-requests")
	{
		// TODO: optimize params
		println!(
			"{:#?}",
			subalfred
				.list_pull_requests(
					list_pull_requests_args.value_of("sha"),
					list_pull_requests_args.value_of("path"),
					list_pull_requests_args.value_of("since"),
					list_pull_requests_args.value_of("until"),
					list_pull_requests_args
						.value_of("thread")
						.unwrap_or("1")
						.parse()
						.unwrap(),
					list_pull_requests_args.is_present("create-issue"),
				)
				.await?
		);
	} else if let Some(list_migrations_args) = app_args.subcommand_matches("list-migrations") {
		// TODO: optimize params
		println!(
			"{:#?}",
			subalfred
				.list_migrations(
					list_migrations_args.value_of("sha"),
					list_migrations_args.value_of("path"),
					list_migrations_args.value_of("since"),
					list_migrations_args.value_of("until"),
					list_migrations_args
						.value_of("thread")
						.unwrap_or("1")
						.parse()
						.unwrap(),
					list_migrations_args.is_present("create-issue"),
				)
				.await?
		);
	} else if let Some(send_rpc_args) = app_args.subcommand_matches("send-rpc") {
		println!(
			"{}",
			serde_json::to_string(
				&Subalfred::send_rpc(
					send_rpc_args
						.value_of("uri")
						.unwrap_or("http://127.0.0.1:9933"),
					subrpcer::rpc(
						send_rpc_args.value_of("method").unwrap(),
						serde_json::from_str::<Value>(
							send_rpc_args.value_of("params").unwrap_or("[]")
						)?,
						send_rpc_args
							.value_of("id")
							.unwrap_or("1")
							.parse::<u32>()
							.unwrap(),
					),
				)
				.await?
				.json::<Value>()?
			)?
		);
	} else if let Some(_) = app_args.subcommand_matches("check-runtime-version") {
		for versions in subalfred.check_runtime_version().await? {
			println!("{:#?}", versions);
		}
	} else if let Some(account_args) = app_args.subcommand_matches("account") {
		let accounts = parse_account(account_args.value_of("account").unwrap());
		let max_width = accounts
			.iter()
			.map(|account| account.0.len())
			.max()
			.unwrap();

		for account in accounts {
			println!("{:>width$}: {}", account.0, account.1, width = max_width);
		}
	} else if let Some(hash_args) = app_args.subcommand_matches("hash") {
		println!(
			"{}",
			hash(
				hash_args.value_of("data").unwrap(),
				hash_args.value_of("hasher").unwrap_or("blake2-128-concat"),
				hash_args.is_present("hex")
			)
		);
	} else if let Some(storage_prefix_args) = app_args.subcommand_matches("storage-prefix") {
		println!(
			"Storage Keys: {}",
			parse_storage_keys(
				storage_prefix_args.value_of("prefix"),
				storage_prefix_args.value_of("item")
			)
		);
	} else if let Some(node_template_args) = app_args.subcommand_matches("node-template") {
		// TODO: output
		Command::new("git")
			.args(&[
				"clone",
				"https://github.com/substrate-developer-hub/substrate-node-template.git",
				node_template_args
					.value_of("name")
					.unwrap_or("substrate-node-template"),
			])
			.output()
			.unwrap();
	}

	Ok(())
}

fn list_app(name: &str) -> App {
	App::new(name)
		.arg(
			Arg::new("sha")
				.long("sha")
				.value_name("BRANCH/SHA")
				.takes_value(true)
				.about(
					"SHA or branch to start listing commits from.\
					Default: the repository’s default branch (usually master).",
				),
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
}

struct Subalfred {
	githuber: Arc<Githuber>,
	project: Project,
}
