use anyhow::Result;

use crate::utils::pathing::config_path;
use crate::handlers::config::CompleteConfig;

mod utils;
mod terminal;
mod reddit;
mod handlers;

const CONF: &str = "https://github.com/Xithrius/terminal-reddit/blob/main/default-config.toml";

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(config_contents) = std::fs::read_to_string(config_path()) {
        let config: CompleteConfig = toml::from_str(config_contents.as_str())?;

        // terminal::draw_terminal_ui().await.unwrap();

        reddit::get_reddit_posts(config.subreddit).await;
    } else {
        println!(
            "Configuration not found. Create a config file at '{}', and see '{}' for an example configuration.",
            config_path(),
            CONF,
        );
    }

    Ok(())
}