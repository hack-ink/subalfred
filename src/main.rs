#![feature(with_options)]

pub mod config;
pub mod substrate;

// --- std ---
use std::fs::{create_dir_all, File};
// --- crates.io ---
use app_dirs2::*;
use async_std::sync::Arc;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githubman::{requests::issues::create_an_issue::CreateAnIssueBuilder, Githubman};
use isahc::ResponseExt;
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
				),
		);
	let app_args = app.get_matches();
	let githubman = Githubman::new(&config.github_oauth_token);
	let substrate = Substrate {
		githubman: Arc::new(githubman.clone()),
	};

	let json: serde_json::Value = githubman
		.send(
			CreateAnIssueBuilder::default()
				.owner(config.substrate_project_owner)
				.repo(config.substrate_project_repo)
				.title("Test Githubman")
				.build()
				.unwrap(),
		)
		.await?
		.json()?;

	#[cfg(feature = "dbg")]
	dbg!(json);

	if let Some(_) = app_args.subcommand_matches("list-repository-tags") {
		substrate.list_repository_tags().await?;
	} else if let Some(_) = app_args.subcommand_matches("list-releases") {
		substrate.list_releases().await?;
	} else if let Some(list_commits_args) = app_args.subcommand_matches("list-commits") {
		substrate.list_commits(list_commits_args).await?;
	} else if let Some(list_migrations_args) = app_args.subcommand_matches("list-migrations") {
		substrate.list_migrations(list_migrations_args).await?;
	}

	Ok(())
}
