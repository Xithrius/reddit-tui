use std::panic::panic_any;

pub enum Status {
    Hot,
    Top,
    New,
    Controversial,
}

pub async fn get_reddit_posts() {
    let response = reqwest::get("https://www.reddit.com/r/memes/.json")
        .await
        .unwrap_or_else(|err| panic_any(err));

    let output: serde_json::Value = response.json()
        .await
        .unwrap_or_else(|err| panic_any(err));

    println!("{:#?}", output);
}
