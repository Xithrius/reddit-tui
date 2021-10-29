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

use crate::handlers::event::{Config, Event, Events, Key};

pub async fn draw_terminal_ui() {
    let mut events = Events::with_config(Config {
        exit_key: Key::Null,
        tick_rate: Duration::from_millis(250),
    })
    .await;

    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend).unwrap();

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

    'outer: loop {
        terminal
            .draw(|frame| {
                let vertical_chunk_constraints = vec![Constraint::Min(1)];

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(vertical_chunk_constraints.as_ref())
                    .split(frame.size());

                let table = Table::new(vec![Row::new(vec!["Row11", "Row12", "Row13"])])
                    .style(Style::default().fg(Color::White))
                    .header(
                        Row::new(vec!["Col1", "Col2", "Col3"])
                            .style(Style::default().fg(Color::Yellow))
                            .bottom_margin(1),
                    )
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("[ Reddit feed ]"),
                    )
                    .widths(&[
                        Constraint::Length(5),
                        Constraint::Length(5),
                        Constraint::Length(10),
                    ])
                    .column_spacing(1)
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                frame.render_widget(table, vertical_chunks[0])
            })
            .unwrap();

        if let Some(Event::Input(key)) = &events.next().await {
            match key {
                Key::Esc => {
                    quitting(terminal);
                    break 'outer;
                }
                Key::Backspace => {}
                Key::Up => {}
                Key::Down => {}
                Key::Left => {}
                Key::Right => {}
                Key::Home => {}
                Key::End => {}
                Key::Delete => {}
                Key::Insert => {}
                Key::PageUp => {}
                Key::PageDown => {}
                Key::Tab => {}
                Key::BackTab => {}
                Key::Enter => {}
                Key::Char(_) => {}
                Key::Ctrl(_) => {}
                Key::Alt(_) => {}
                Key::F(_) => {}
                Key::Null => {}
            }
        }
    }
}
