use color_eyre::eyre::bail;

use crate::handlers::config::CompleteConfig;

#[derive(Debug, Clone)]
pub struct Reddit {
    pub client: reqwest::Client,
    pub user_agent: String,
    pub bearer_token: String,
}

impl Reddit {
    pub async fn new(config: &CompleteConfig) -> Result<Self, Error> {
        let user_agent = format!("tui/v{} by {}", env!("CARGO_PKG_VERSION"), config.username);

        let client = reqwest::Client::builder().build().unwrap();

        let request_form = reqwest::multipart::Form::new()
            .text("User-Agent", user_agent.to_string())
            .text("grant_type", "password")
            .text("username", config.username.to_string())
            .text("password", config.password.to_string());

        let response = client
            .post("https://www.reddit.com/api/v1/access_token")
            .multipart(request_form)
            .basic_auth(
                config.client_id.to_string(),
                Some(config.client_secret.to_string()),
            )
            .send()
            .await
            .unwrap();

        if response.status() != 200 {
            bail!(
                "Could not authenticate through the Reddit API. Status: {}.",
                response.status()
            );
        } else {
            match response.json().await {
                Ok(data) => {
                    let auth_info: serde_json::Value = data;

                    Ok(Reddit {
                        client,
                        user_agent,
                        bearer_token: format!("bearer {}", auth_info["access_token"])
                            .replace("\"", ""),
                    })
                }
                _ => bail!("Could not parse Reddit authentication data."),
            }
        }
    }

    pub async fn get(self, url: String) -> Result<serde_json::Value, Error> {
        let response = self
            .client
            .get(url)
            .header("Authorization", self.bearer_token)
            .header("User-Agent", self.user_agent)
            .send()
            .await;

        match response {
            Ok(data) => {
                let info: serde_json::Value = data.json().await.unwrap();

                Ok(info)
            }
            Err(err) => bail!("Failed to deserialize response data. Error: {}", err),
        }
    }
}
