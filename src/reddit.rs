use std::collections::VecDeque;

use reqwest::Client;
use serde::Deserialize;

use crate::handlers::config::CompleteConfig;

const BASE_URL: &str = "https://www.reddit.com";

#[derive(Deserialize, Debug)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub url: String,
    pub permalink: String,
}

impl Post {
    pub fn new(author: String, title: String, url: String, permalink: String) -> Self {
        Post {
            author,
            title,
            url,
            permalink,
        }
    }
}

pub async fn get_reddit_posts(
    client: Client,
    request_form: reqwest::multipart::Form,
    config: CompleteConfig,
) -> VecDeque<Post> {
    let response: serde_json::Value = client
        .get(format!(
            "{}/r/{}/.json",
            BASE_URL,
            config.subreddit.to_string()
        ))
        .multipart(request_form)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let mut posts = VecDeque::new();

    // for item in response["data"]["children"].as_array().unwrap() {
    //     let tmp = &item["data"];

    //     posts.push_front(Post::new(
    //         tmp["author"].to_string(),
    //         tmp["title"].to_string(),
    //         tmp["url"].to_string(),
    //         tmp["permalink"].to_string(),
    //     ));
    // }

    posts
}
