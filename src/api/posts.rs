use std::{collections::VecDeque, time::Duration};

use anyhow::{Error, Result};

use crate::handlers::{app::Reddit, config::CompleteConfig, post::Post};

pub async fn get_reddit_posts(
    reddit_client: Reddit,
    config: CompleteConfig,
) -> Result<VecDeque<Post>, Error> {
    let url_formatter = |after: Option<String>| {
        format!(
            "https://oauth.reddit.com/r/{}/{}/.json?t={}&limit=100{}",
            config.subreddit,
            config.status,
            config.timespan,
            match after {
                Some(page_hash) => format!("&after={}", page_hash).replace("\"", ""),
                None => "".to_string(),
            }
        )
    };

    let mut posts = VecDeque::new();

    let mut after: Option<String> = None;

    'outer: loop {
        let info = reddit_client
            .clone()
            .get(url_formatter(after.clone()))
            .await
            .expect("Failed to get posts.");

        after = Some(info["data"]["after"].to_string());

        let post_list = info["data"]["children"].as_array().unwrap();

        if post_list.is_empty() {
            break 'outer;
        }

        for item in post_list {
            let tmp_data = &item["data"];

            if tmp_data["ups"].to_string().parse::<i32>().unwrap() < 500 {
                break 'outer;
            }

            posts.push_front(Post::new(
                tmp_data["author"].to_string(),
                tmp_data["title"].to_string(),
                tmp_data["url"].to_string(),
                format!("https://reddit.com{}", tmp_data["permalink"].to_string()),
                tmp_data["url"].to_string(),
            ));
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(posts)
}
