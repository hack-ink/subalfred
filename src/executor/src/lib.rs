#[cfg(feature = "checker")]
mod checker;
mod config;
#[cfg(feature = "crypto")]
mod crypto;
#[cfg(feature = "node")]
mod node;

// crates.io
use anyhow::Result as AnyResult;
use async_std::sync::Arc;
use config::Config;
use githuber::Githuber;
use once_cell::sync::Lazy;

pub static EXECUTOR: Lazy<Arc<Executor>> = Lazy::new(|| Arc::new(Executor::new().unwrap()));

pub struct Executor {
	pub githuber: Arc<Githuber>,
}
impl Executor {
	pub fn new() -> AnyResult<Self> {
		let Config { github_oauth_token } = Config::load()?;

		Ok(Self {
			githuber: Arc::new(Githuber::new(github_oauth_token)),
		})
	}
}
