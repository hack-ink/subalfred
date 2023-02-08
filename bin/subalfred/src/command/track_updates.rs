// crates.io
use clap::Args;
// subalfred
use crate::prelude::*;
use subalfred_core::github::{
	substrate::{self, WatchedLabels, WatchedPullRequests},
	PullRequest,
};

/// Track the updates.
///
/// This command require a `GITHUB_TOKEN` environment variable to be set.
/// It will list all the commits between the `from` and `to` GitHub ref.
///
/// The output is in markdown format.
#[derive(Debug, Args)]
pub(crate) struct TrackUpdatesCmd {
	/// Target repository.
	///
	/// e.g. paritytech/substrate
	#[arg(value_name = "OWNER/REPOSITORY")]
	repository: String,
	/// Release starting from.
	#[arg(long, required = true, value_name = "VERSION")]
	from: String,
	/// Release updating to.
	#[arg(long, required = true, value_name = "VERSION")]
	to: String,
}
impl TrackUpdatesCmd {
	async fn run_(&self) -> Result<Vec<PullRequest>> {
		let Self { repository, from, to } = self;
		let (owner, repo) = repository.split_once('/').ok_or_else(|| {
			anyhow::anyhow!("[cli::track_updates] invalid repository, {repository:?}")
		})?;

		Ok(substrate::track_updates(owner, repo, &format!("{from}...{to}")).await?)
	}

	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		let ps = self.run_().await?;

		println!("### Track updates");
		println!(
			"```\n\
			repository: {}\n\
			commits   : {}\n\
			command   : subalfred track-updates {} --from {} --to {}\n\
			```",
			self.repository,
			ps.len(),
			self.repository,
			self.from,
			self.to,
		);
		println!("> https://github.com/{}/compare/{}...{}", self.repository, self.from, self.to);
		println!("### All");

		ps.iter().for_each(|p| {
			println!(
				"- [{}]({}){}",
				p.title,
				p.html_url,
				if p.labels.is_empty() {
					String::new()
				} else {
					format!(
						" - {}",
						p.labels
							.iter()
							.map(|label| format!("`{}`", label.name))
							.collect::<Vec<_>>()
							.join(", ")
					)
				}
			)
		});

		let w = WatchedPullRequests::from(ps);

		println!("### Watched labels");

		WatchedLabels::all()
			.into_iter()
			.zip(w.all().into_iter())
			.filter(|(_, ps)| !ps.is_empty())
			.for_each(|(l, ps)| {
				println!("- #### {l}");

				ps.into_iter().for_each(|p| println!("\t- [{}]({})", p.title, p.html_url));
			});

		Ok(())
	}
}
