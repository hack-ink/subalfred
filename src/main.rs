pub mod substrate;

// --- crates.io ---
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use githubman::{action::commits::list_commits::ListCommitsBuilder, GithubMan};
use isahc::ResponseExt;

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let app = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.subcommand(App::new("list-commits").about("").arg(
			Arg::new("sha").long("sha").takes_value(true).about(
				"SHA or branch to start listing commits from. \
				Default: the repository’s default branch (usually master).",
			),
		))
		.subcommand(
			App::new("list-migrations")
				.about("")
				.arg(Arg::new("sha").long("sha").takes_value(true))
				.about(
					"SHA or branch to start listing commits from.\
					Default: the repository’s default branch (usually master).",
				)
				.arg(
					Arg::new("path")
						.long("path")
						.takes_value(true)
						.about("Only commits containing this file path will be returned."),
				)
				.arg(Arg::new("since").long("since").takes_value(true).about(
					"Only show notifications updated after the given time. \
					This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.",
				))
				.arg(Arg::new("until").long("until").takes_value(true).about(
					"Only commits before this date will be returned. \
					This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.",
				)),
		);
	let app_args = app.get_matches();
	let github_man = GithubMan::new();

	if let Some(list_commits_args) = app_args.subcommand_matches("list-commits") {
		let sha = if let Some(sha) = list_commits_args.value_of("sha") {
			sha
		} else {
			"master"
		};
		let response: serde_json::Value = github_man
			.request(
				ListCommitsBuilder::default()
					.owner("paritytech")
					.repo("substrate")
					.sha(sha)
					.per_page(1u32)
					.build()
					.unwrap(),
			)
			.await?
			.json()?;

		dbg!(response);
	}

	Ok(())
}
