use color_eyre::eyre::{Result, WrapErr};

use crate::handlers::{app::App, config::CompleteConfig};

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

    terminal::ui_driver(config, app).await;

    std::process::exit(0)
}
