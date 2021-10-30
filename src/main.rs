use anyhow::Result;
use handlers::app::Reddit;

use crate::handlers::config::CompleteConfig;

mod handlers;
mod reddit;
mod terminal;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    match CompleteConfig::new() {
        Ok(config) => {
            if let Ok(reddit_client) = Reddit::new(&config).await {
                match reddit::get_reddit_posts(reddit_client, config).await {
                    Ok(posts) => {
                        // terminal::draw_terminal_ui(posts).await;
                        println!("Finished, with a total of {} posts.", posts.len());
                        // for p in posts.iter() {
                        //     println!("{}", p.title);
                        // }
                    }
                    Err(err) => println!("Failed to request posts from Reddit. Error: {}", err),
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
