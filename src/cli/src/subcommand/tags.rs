// --- crates.io ---
use async_std::task;
use clap::{clap, clap::arg_enum};
// --- hack-ink ---
use crate::{cli::Run, AnyResult, Subalfred};

arg_enum! {
	#[derive(Debug)]
	pub enum Project {
		Cumulus,
		Frontier,
		Polkadot,
		Substrate,
	}
}
impl Project {
	fn github_repository(&self) -> (&str, &str) {
		match self {
			Project::Cumulus => ("paritytech", "cumulus"),
			Project::Frontier => ("paritytech", "frontier"),
			Project::Polkadot => ("paritytech", "frontier"),
			Project::Substrate => ("paritytech", "substrate"),
		}
	}
}

// TODO: flatten this to be a shared object
#[derive(Debug, Parser)]
pub struct TagsCmd {
	#[clap(
		help = "Specific project (non case sensitive)",
		short,
		long,
		case_insensitive = true,
		required = true,
		takes_value = true,
		possible_values = &Project::variants(),
		value_name = "PROJECT"
	)]
	project: Project,
}
impl Run for TagsCmd {
	fn run(&self) -> AnyResult<()> {
		let subalfred = Subalfred::init();
		let (owner, repo) = self.project.github_repository();

		println!("{:#?}", task::block_on(subalfred.list_tags(owner, repo))?);

		Ok(())
	}
}
