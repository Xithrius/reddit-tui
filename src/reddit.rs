use std::panic::panic_any;
use std::collections::VecDeque;

const BASE_URL: &str = "https://www.reddit.com/";

// pub enum Status {
//     Hot,
//     Top,
//     New,
//     Controversial,
// }

pub struct Post {
    author: String,
    title: String,
    url: String,
    permalink: String,
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

pub async fn get_reddit_posts(subreddit: String) -> VecDeque<Post> {
    let response = reqwest::get(format!("r/{}/{}/.json", BASE_URL, subreddit))
        .await
        .unwrap_or_else(|err| panic_any(err));

    let output: serde_json::Value = response.json()
        .await
        .unwrap_or_else(|err| panic_any(err));

    let mut posts = VecDeque::new();

    for item in output["data"]["children"].as_array().unwrap() {
        let tmp = &item["data"];

        posts.push_front(Post::new(
            tmp["author"].to_string(),
            tmp["title"].to_string(),
            tmp["url"].to_string(),
            tmp["permalink"].to_string(),
        ));
    }

    posts
}
