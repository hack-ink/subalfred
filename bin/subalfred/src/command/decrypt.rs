// std
use std::path::PathBuf;
// crates.io
use clap::Args;
// subalfred
use crate::prelude::*;
use subalfred_core::{key::KeystoreJson, system};

/// Calculate the storage key of the storage item.
#[derive(Debug, Args)]
pub(crate) struct DecryptCmd {
	/// Path to the JSON keystore file.
	#[arg(value_name = "PATH")]
	path: PathBuf,
}
impl DecryptCmd {
	pub(crate) fn run(&self) -> Result<()> {
		let Self { path } = self;
		let keystore = system::read_file_to_struct::<_, KeystoreJson>(path)?;

		loop {
			let password = rpassword::prompt_password("Keystore password:")?;

			if let Ok(secret_key) = keystore.decrypt(&password) {
				println!("{}", array_bytes::bytes2hex("0x", secret_key));

				return Ok(());
			} else {
				println!("Wrong password!");
			}
		}
	}
}
