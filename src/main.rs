use anyhow::Result;

use crate::handlers::config::CompleteConfig;

mod handlers;
mod reddit;
mod terminal;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    match CompleteConfig::new() {
        Ok(config) => {
            // <platform>:<app ID>:<version string> (by /u/<reddit username>)
            let user_agent = format!("tui/v{} by {}", env!("CARGO_PKG_VERSION"), config.username);

            println!("{}", user_agent);

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
            } else if let Ok(data) = response.json().await {
                let auth_info: serde_json::Value = data;

                let token = format!("bearer {}", auth_info["access_token"]).replace("\"", "");

                let posts = reddit::get_reddit_posts(client, token, user_agent, config).await;

                terminal::draw_terminal_ui().await;
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
