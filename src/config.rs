//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

//use crate::database::DatabaseConnection;
use crate::database::DatabaseConnection;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
	pub redis_url: String,
	pub database: DatabaseConnection,
	pub database_url: String,
	pub rust_backtrace: u8,
	pub rust_log: String,
	pub server: String,
	pub mail_user: String,
	pub mail_pwd: String,
	pub mail_receiver: String,
	pub use_db: u8,
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
	pub static ref CONFIG: Config = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
	dotenv().ok();

	match envy::from_env::<Config>() {
		Ok(config) => config,
		Err(error) => panic!("Configuration Error: {:#?}", error),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_gets_a_config() {
		let config = get_config();
		assert_ne!(config.server, "".to_string());
	}

	#[test]
	fn it_gets_a_config_from_the_lazy_static() {
		let config = &CONFIG;
		assert_ne!(config.server, "".to_string());
	}
}
