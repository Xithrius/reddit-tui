use anyhow::Result;

use crate::handlers::config::CompleteConfig;
use crate::utils::pathing::config_path;

mod handlers;
mod reddit;
mod terminal;
mod utils;

const CONF: &str = "https://github.com/Xithrius/terminal-reddit/blob/main/default-config.toml";

#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(config_contents) = std::fs::read_to_string(config_path()) {
        let config: CompleteConfig = toml::from_str(config_contents.as_str())?;

        // <platform>:<app ID>:<version string> (by /u/<reddit username>)
        let user_agent = format!(
            "{}/v{} by Xithrius",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        let client = reqwest::Client::builder().build().unwrap();

        let form = reqwest::multipart::Form::new()
            .text("User-Agent", user_agent.to_string())
            .text("grant_type", "password")
            .text("username", config.username.to_string())
            .text("password", config.password.to_string());

        let response = client
            .post("https://www.reddit.com/api/v1/access_token")
            .multipart(form)
            .basic_auth(
                config.client_id.to_string(),
                Some(config.client_secret.to_string()),
            )
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            println!(
                "{}",
                format!(
                    "Could not authenticate through the Reddit API. Status: {}.",
                    response.status()
                )
            );
        } else {
            let some_json: serde_json::Value = response.json().await.unwrap();

            let token = format!("bearer {}", some_json["access_token"]);

            let request_form = reqwest::multipart::Form::new()
                .text("Authorization", token)
                .text("User-Agent", user_agent);

            let posts = reddit::get_reddit_posts(client, request_form, config.clone()).await;

            for post in posts {
                println!("{:?}", post);
            }
        }

        // terminal::draw_terminal_ui().await.unwrap();
    } else {
        println!(
            "Configuration not found. Create a config file at '{}', and see '{}' for an example configuration.",
            config_path(),
            CONF,
        );
    }

    Ok(())
}
