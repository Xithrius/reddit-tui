use std::time::Duration;

use crate::{
    commands::{init_terminal, quit_terminal, reset_terminal},
    handlers::{
        app::App,
        config::CompleteConfig,
        event::{Config, Event, Events, Key},
    },
    ui::draw_ui,
};

pub async fn ui_driver(config: CompleteConfig, mut app: App) {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal();
        original_hook(panic);
    }));

    let mut events = Events::with_config(Config {
        exit_key: Key::Null,
        tick_rate: Duration::from_millis(config.terminal.tick_delay),
    })
    .await;

    let mut terminal = init_terminal(&config.frontend);

    terminal.clear().unwrap();

    loop {
        terminal
            .draw(|frame| draw_ui(frame, &mut app, &config))
            .unwrap();

        if let Some(Event::Input(key)) = &events.next().await {
            match key {
                Key::Char('q') => {
                    quit_terminal(terminal);

                    break;
                }
                Key::Char('p') => {
                    panic!("Manual panic triggered.");
                }
                _ => {}
            }
        }
    }

    reset_terminal();
}
