#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::unused_self,
    clippy::future_not_send,
    clippy::suboptimal_flops
)]

use api::{auth::Reddit, posts::get_reddit_posts};
use color_eyre::eyre::{Result, WrapErr};

use crate::handlers::{app::App, config::CompleteConfig};

mod api;
mod commands;
mod handlers;
mod terminal;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let config = CompleteConfig::new()
        .wrap_err("Configuration error.")
        .unwrap();

    let app = App::new();

    let reddit = Reddit::new(&config.reddit).await.unwrap();

    get_reddit_posts(reddit, config.reddit).await.unwrap();

    // terminal::ui_driver(config, app).await;

    std::process::exit(0)
}
