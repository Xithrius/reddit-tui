use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CompleteConfig {
    pub subreddit: String,
    pub status: String,
}
