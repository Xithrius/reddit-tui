use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CompleteConfig {
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: String,
    pub subreddit: String,
    pub status: String,
}
