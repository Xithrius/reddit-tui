use anyhow::Result;

mod utils;
mod terminal;
mod reddit;

#[tokio::main]
async fn main() -> Result<()> {
    // terminal::draw_terminal_ui().await.unwrap();

    reddit::get_reddit_posts().await;

    Ok(())
}
