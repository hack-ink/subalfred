#![feature(with_options)]

pub mod config;
pub mod substrate;

// --- std ---
use std::env;
// --- crates.io ---
use anyhow::Result as AnyResult;
use app_dirs2::AppInfo;
use async_std::sync::Arc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githuber::Githuber;
use isahc::ResponseExt;
use serde_json::Value;
// --- subalfred ---
use crate::config::Project;

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
				.about("")
				.long("long")
				.short('l')
				.takes_value(true)
				.value_name("TARGET")
				.global(true),
		)
		.subcommand(App::new("list-tags").about(""))
		.subcommand(App::new("list-releases").about(""))
		.subcommand(list_app("list-commits").about(""))
		.subcommand(
			list_app("list-pull-requests")
				.about("")
				.arg(
					Arg::new("thread")
						.about("")
						.long("thread")
						.takes_value(true)
						.value_name("COUNT"),
				)
				.arg(Arg::new("create-issue").about("").long("create-issue")),
		)
		.subcommand(
			list_app("list-migrations")
				.about("")
				.arg(
					Arg::new("thread")
						.about("")
						.long("thread")
						.takes_value(true)
						.value_name("COUNT"),
				)
				.arg(Arg::new("create-issue").about("").long("create-issue")),
		)
		.subcommand(
			App::new("send-rpc")
				.about("")
				.arg(
					Arg::new("uri")
						.about("")
						.long("uri")
						.takes_value(true)
						.value_name("URI"),
				)
				.arg(
					Arg::new("method")
						.about("")
						.long("method")
						.required(true)
						.takes_value(true)
						.value_name("METHOD"),
				)
				.arg(
					Arg::new("params")
						.about("")
						.long("params")
						.takes_value(true)
						.value_name("[PARAM]"),
				)
				.arg(
					Arg::new("id")
						.about("")
						.long("id")
						.takes_value(true)
						.value_name("ID"),
				),
		)
		.subcommand(App::new("check-runtime-version").about(""))
		.subcommand(
			App::new("metadata")
				.about("")
				.arg(
					Arg::new("uri")
						.about("")
						.long("uri")
						.takes_value(true)
						.value_name("URI"),
				)
				.arg(
					Arg::new("list-module")
						.about("")
						.long("list-module")
						.conflicts_with("list-storage-keys"),
				)
				.arg(
					Arg::new("list-storage-keys")
						.about("")
						.long("list-storage-keys")
						.conflicts_with("list-module"),
				),
		)
		.subcommand(
			App::new("account").about("").arg(
				Arg::new("account")
					.about("")
					.required(true)
					.takes_value(true)
					.value_name("PUBLIC KEY/SS58 ADDRESS"),
			),
		)
		.subcommand(
			App::new("hash")
				.about("")
				.arg(
					Arg::new("data")
						.about("")
						.required(true)
						.takes_value(true)
						.value_name("VALUE"),
				)
				.arg(
					Arg::new("hasher")
						.about("")
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
						.value_name("HASHER"),
				)
				.arg(Arg::new("hex").about("").long("hex")),
		)
		.subcommand(
			// TODO: handle instance
			App::new("storage-key")
				.about("")
				.arg(
					Arg::new("prefix")
						.about("")
						.long("prefix")
						.conflicts_with("list")
						.takes_value(true)
						.value_name("NAME"),
				)
				.arg(
					Arg::new("item")
						.about("")
						.long("item")
						.conflicts_with("list")
						.takes_value(true)
						.value_name("NAME"),
				),
		)
		.subcommand(
			App::new("node-template").about("").arg(
				Arg::new("name")
					.about("")
					.long("name")
					.takes_value(true)
					.value_name("NAME"),
			),
		)
		.subcommand(
			App::new("pallet-template")
				.about("")
				.arg(
					Arg::new("name")
						.about("")
						.long("name")
						.takes_value(true)
						.value_name("NAME"),
				)
				.arg(Arg::new("multi-instance").about("").long("multi-instance"))
				.arg(
					Arg::new("dependency-path")
						.about("")
						.long("dependency-path")
						.takes_value(true)
						.value_name("PATH"),
				)
				.arg(
					Arg::new("dependency-git")
						.about("")
						.long("dependency-git")
						.takes_value(true)
						.value_name("GIT"),
				)
				.arg(
					Arg::new("dependency-commit")
						.about("")
						.long("dependency-commit")
						.takes_value(true)
						.value_name("SHA"),
				)
				.arg(
					Arg::new("dependency-branch")
						.about("")
						.long("dependency-branch")
						.takes_value(true)
						.value_name("SHA"),
				)
				.arg(
					Arg::new("dependency-tag")
						.about("")
						.long("dependency-tag")
						.takes_value(true)
						.value_name("TAG"),
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
	if app_args.subcommand_matches("list-tags").is_some() {
		println!("{:#?}", subalfred.list_tags().await?);
	} else if app_args.subcommand_matches("list-releases").is_some() {
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
		let uri = send_rpc_args
			.value_of("uri")
			.unwrap_or("http://127.0.0.1:9933");
		let params =
			|| serde_json::from_str::<Value>(send_rpc_args.value_of("params").unwrap_or("[]"));
		let rpc = match send_rpc_args.value_of("method").unwrap() {
			"author_submitAndWatchExtrinsic" | "submitAndWatchExtrinsic" => {
				subrpcer::author::submit_and_watch_extrinsic_with_params(params()?)
			}
			"chain_getBlockHash" | "getBlockHash" => {
				subrpcer::chain::get_block_hash_with_raw_params(params()?)
			}
			"state_getRuntimeVersion" | "getRuntimeVersion" => {
				subrpcer::state::get_runtime_version()
			}
			"state_getMetadata" | "getMetadata" => subrpcer::state::get_metadata(),
			"state_getStorage" | "getStorage" => {
				subrpcer::state::get_storage_with_raw_params(params()?)
			}
			method => subrpcer::rpc(
				method,
				params()?,
				send_rpc_args
					.value_of("id")
					.unwrap_or("1")
					.parse::<u32>()
					.unwrap(),
			),
		};

		println!(
			"{}",
			serde_json::to_string(&subrpcer::send_rpc(uri, rpc).await?.json::<Value>()?)?
		);
	} else if app_args
		.subcommand_matches("check-runtime-version")
		.is_some()
	{
		for versions in subalfred.check_runtime_version().await? {
			println!("{:#?}", versions);
		}
	} else if let Some(metadata_args) = app_args.subcommand_matches("metadata") {
		let uri = metadata_args
			.value_of("uri")
			.unwrap_or("http://127.0.0.1:9933");

		if metadata_args.is_present("list-module") {
			println!("{:#?}", Subalfred::list_module(uri).await?);
		} else if metadata_args.is_present("list-storage-keys") {
			println!("{:#?}", Subalfred::list_storage_keys(uri).await?);
		}
	} else if let Some(account_args) = app_args.subcommand_matches("account") {
		let accounts = Subalfred::accounts(account_args.value_of("account").unwrap());
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
			Subalfred::hash(
				hash_args.value_of("data").unwrap(),
				hash_args.value_of("hasher").unwrap_or("blake2-128-concat"),
				hash_args.is_present("hex")
			)
		);
	} else if let Some(storage_prefix_args) = app_args.subcommand_matches("storage-key") {
		println!(
			"Storage Keys: {}",
			Subalfred::storage_keys(
				storage_prefix_args.value_of("prefix"),
				storage_prefix_args.value_of("item")
			)
		);
	} else if let Some(node_template_args) = app_args.subcommand_matches("node-template") {
		// TODO: output
		Subalfred::node_template(
			node_template_args
				.value_of("name")
				.unwrap_or("substrate-node-template"),
		);
	} else if let Some(pallet_template_args) = app_args.subcommand_matches("pallet-template") {
		Subalfred::pallet_template(
			pallet_template_args
				.value_of("name")
				.unwrap_or("substrate-pallet-template"),
			pallet_template_args.is_present("multi-instance"),
			pallet_template_args.value_of("dependency-path"),
			pallet_template_args.value_of("dependency-git"),
			pallet_template_args.value_of("dependency-commit"),
			pallet_template_args.value_of("dependency-branch"),
			pallet_template_args.value_of("dependency-tag"),
		);
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
