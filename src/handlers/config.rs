use anyhow::{bail, Error, Result};
use serde::Deserialize;

use crate::utils::pathing::config_path;

#[derive(Deserialize, Clone)]
pub struct CompleteConfig {
    /// Username of the current person using the program
    pub username: String,
    /// Password of the user
    pub password: String,
    /// Client ID of the script application
    pub client_id: String,
    /// The secret of the script application
    pub client_secret: String,
    /// The subreddit that the user wants to browse
    pub subreddit: String,
    /// The status of the browser (top, hot, new, controversial)
    pub status: String,
    /// The amount of time in which the posts had been made (day, week, month, year, all)
    pub timespan: String,
}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        if let Ok(config_contents) = std::fs::read_to_string(config_path()) {
            let config: CompleteConfig = toml::from_str(config_contents.as_str()).unwrap();

            Ok(config)
        } else {
            bail!(
                "Configuration not found. Create a config file at '{}', and see '{}' for an example configuration.",
                config_path(),
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}
