use std::collections::VecDeque;
use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};

use crate::handlers::state::TableEvents;
use crate::{
    handlers::{
        app::App,
        config::CompleteConfig,
        event::{Config, Event, Events, Key},
    },
    ui::draw_ui,
};

fn reset_terminal() {
    disable_raw_mode().unwrap();

    execute!(stdout(), LeaveAlternateScreen).unwrap();
}

fn init_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend).unwrap()
}

pub async fn ui_driver(mut config: CompleteConfig, mut app: App) {
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

    let mut terminal = init_terminal();

    terminal.clear().unwrap();

    let quitting = |mut terminal: Terminal<CrosstermBackend<Stdout>>| {
        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        terminal.show_cursor().unwrap();
    };

    let mut table_events = TableEvents::new(VecDeque::new());

    'outer: loop {
        terminal
            .draw(|frame| draw_ui(frame, &mut app, &config, &mut table_events))
            .unwrap();

        if let Some(Event::Input(key)) = &events.next().await {
            match key {
                Key::Char('q') => {
                    quitting(terminal);
                    break 'outer;
                }
                _ => {}
            }
        }
    }
}
