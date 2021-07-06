#![feature(with_options)]

pub mod config;
pub mod substrate;

// --- std ---
use std::env;
// --- crates.io ---
use anyhow::Result as AnyResult;
use app_dirs2::AppInfo;
use async_std::sync::Arc;
use clap::{App, Arg};
use githuber::Githuber;
use isahc::AsyncReadResponseExt;
use serde_json::Value;
// --- subalfred ---
use crate::config::Project;

const APP_INFO: AppInfo = AppInfo {
	name: clap::crate_name!(),
	author: clap::crate_authors!(),
};

#[async_std::main]
async fn main() -> AnyResult<()> {
	// TODO: about
	// TODO: --json for output
	let app = App::new(clap::crate_name!())
		.version(concat!(
			env!("VERGEN_BUILD_SEMVER"),
			"-",
			env!("VERGEN_GIT_SHA_SHORT"),
			"-",
			env!("VERGEN_CARGO_TARGET_TRIPLE"),
		))
		.author(clap::crate_authors!())
		.about(clap::crate_description!())
		.arg(
			Arg::new("log")
				.about("Enable/Print logs same as the `RUST_LOG`")
				.long("log")
				.short('l')
				.takes_value(true)
				.value_name("TARGET")
				.global(true),
		)
		.subcommand(App::new("list-tags").about("List all the tags of Substrate"))
		.subcommand(App::new("list-releases").about("List all the releases of Substrate"))
		.subcommand(list_app("list-commits").about("List the specific commits of Substrate"))
		.subcommand(
			list_app("list-pull-requests")
				.about("List the specific pull requests of Substrate")
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
				.about("List the specific pull requests which contains runtime migration(s) of Substrate")
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
				.about("Send RPC to the node")
				.arg(
					Arg::new("uri")
						.about("")
						.long("uri")
						.takes_value(true)
						.value_name("URI"),
				)
				.arg(
					Arg::new("method")
						.about("Support these styles (non case sensitive):\n    - state_getRuntimeVersion\n    - getRuntimeVersion")
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
		.subcommand(App::new("check-runtime-version").about("Fetch and check the given node's runtime version"))
		.subcommand(
			App::new("metadata")
				.about("Read and parse the given node's metadata")
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
			App::new("account").about("Convert the SS58/public to public/SS58").arg(
				Arg::new("account")
					.about("")
					.required(true)
					.takes_value(true)
					.value_name("PUBLIC KEY/SS58 ADDRESS"),
			),
		)
		.subcommand(
			App::new("hash")
				.about("Hash the given data with specific hasher")
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
				.about("Calculate the storage key for the storage item/prefix")
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
			App::new("node-template").about("Create a node template in current dir").arg(
				Arg::new("name")
					.about("")
					.long("name")
					.takes_value(true)
					.value_name("NAME"),
			),
		)
		.subcommand(
			App::new("pallet-template")
				.about("Create a pallet template in current dir")
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

	if let Some(logs) = app_args.value_of("log") {
		env::set_var("RUST_LOG", logs);
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
		let method = send_rpc_args.value_of("method").unwrap();
		let rpc = match method.to_lowercase().as_str() {
			"author_submitandwatchextrinsic" | "submitandwatchextrinsic" => {
				subrpcer::author::submit_and_watch_extrinsic_with_raw_params(params()?)
			}
			"chain_getblockhash" | "getblockhash" => {
				subrpcer::chain::get_block_hash_with_raw_params(params()?)
			}
			"state_getruntimeversion" | "getruntimeversion" => {
				subrpcer::state::get_runtime_version()
			}
			"state_getmetadata" | "getmetadata" => subrpcer::state::get_metadata(),
			"state_getstorage" | "getstorage" => {
				subrpcer::state::get_storage_with_raw_params(params()?)
			}
			_ => subrpcer::rpc(
				send_rpc_args
					.value_of("id")
					.unwrap_or("1")
					.parse::<u32>()
					.unwrap(),
				method,
				params()?,
			),
		};

		tracing::trace!("{}", serde_json::to_string_pretty(&rpc)?);
		println!("{}", subrpcer::send_rpc(uri, rpc).await?.text().await?);
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
			Subalfred::storage_key(
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
