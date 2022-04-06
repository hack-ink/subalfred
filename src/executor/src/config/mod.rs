mod template;

// std
use std::{
	fs::File,
	io::{Read, Write},
};
// crates.io
use app_dirs2::{AppDataType, AppInfo};
use serde::{Deserialize, Serialize};
// hack-ink
use crate::AnyResult;
use template::TEMPLATE;

const APP_INFO: AppInfo = AppInfo {
	name: clap::crate_name!(),
	author: clap::crate_authors!(),
};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
	pub github_oauth_token: String,
}
impl Config {
	pub fn load() -> AnyResult<Self> {
		let config_path =
			app_dirs2::app_root(AppDataType::UserConfig, &APP_INFO)?.join("config.toml");

		Ok(if let Ok(mut config) = File::open(&config_path) {
			let mut bytes = vec![];

			config.read_to_end(&mut bytes)?;
			toml::from_slice(&bytes)?
		} else {
			let config = Config::default();

			write!(File::create(config_path)?, "{}", TEMPLATE)?;

			config
		})
	}
}
