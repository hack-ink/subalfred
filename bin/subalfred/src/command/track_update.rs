// crates.io
use clap::Args;
// hack-ink
use crate::prelude::*;
use subalfred_core::github::{self, PullRequest};

/// Track the updates.
///
/// Basically, it compares two commits and return the associated pull requests.
///
/// The output is in markdown format.
#[derive(Debug, Args)]
pub(crate) struct TrackUpdateCmd {
	/// Target repository.
	///
	/// e.g. paritytech/substrate
	#[arg(required = true, value_name = "OWNER/REPOSITORY")]
	repository: String,
	/// Release starting from.
	#[arg(long, required = true, value_name = "VERSION")]
	from: String,
	/// Release updating to.
	#[arg(long, required = true, value_name = "VERSION")]
	to: String,
}
impl TrackUpdateCmd {
	async fn run_(&self) -> Result<Vec<PullRequest>> {
		let Self { repository, from, to } = self;
		let (owner, repo) = repository.split_once('/').ok_or_else(|| {
			anyhow::anyhow!("[cli::track_update] invalid repository, {repository:?}")
		})?;

		Ok(github::track_update(owner, repo, &format!("{from}...{to}")).await?)
	}

	#[tokio::main]
	pub(crate) async fn run(&self) -> Result<()> {
		self.run_().await?.into_iter().for_each(|PullRequest { title, html_url, labels }| {
			println!(
				"- [{title}]({html_url}){}",
				if labels.is_empty() {
					String::new()
				} else {
					format!(
						" - {}",
						labels
							.into_iter()
							.map(|label| format!("`{}`", label.name))
							.collect::<Vec<_>>()
							.join(", ")
					)
				}
			)
		});

		Ok(())
	}
}
