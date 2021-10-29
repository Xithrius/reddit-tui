use std::collections::VecDeque;

use reqwest::Client;
use serde::Deserialize;

use anyhow::{bail, Error, Result};

use crate::handlers::config::CompleteConfig;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub url: String,
    pub permalink: String,
    pub image_url: String,
}

impl Post {
    pub fn new(author: String, title: String, url: String, permalink: String, image_url: String) -> Self {
        Post {
            author,
            title,
            url,
            permalink,
            image_url,
        }
    }
}

pub async fn get_reddit_posts(
    client: Client,
    token: String,
    user_agent: String,
    config: CompleteConfig,
) -> Result<VecDeque<Post>, Error> {
    let url = format!(
        "https://oauth.reddit.com/r/{}/{}/?t={}.json",
        config.subreddit, config.status, config.timespan
    );

    let response = client
        .get(url)
        .header("Authorization", token)
        .header("User-Agent", user_agent)
        .send()
        .await;

    match response {
        Ok(data) => {
            let info: serde_json::Value = data.json().await.unwrap();

            let mut posts = VecDeque::new();

            for item in info["data"]["children"].as_array().unwrap() {
                let tmp = &item["data"];

                posts.push_front(Post::new(
                    tmp["author"].to_string(),
                    tmp["title"].to_string(),
                    tmp["url"].to_string(),
                    format!("https://reddit.com{}", tmp["permalink"].to_string()),
                    tmp["url"].to_string(),
                ));
            }

            Ok(posts)
        }
        Err(err) => bail!(format!("Request to Reddit API failed. Error: {}", err)),
    }
}
