use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, layout::Constraint, Terminal};
use tui::layout::{Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Cell, Row, Table, Borders};

use crate::utils::{app::App, event};

pub async fn draw_terminal_ui() -> Result<()> {
    let mut events = event::Events::with_config(event::Config {
        exit_key: KeyCode::Null,
        tick_rate: Duration::from_millis(250),
    })
        .await;

    let app = App::new(25);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

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
        // if let Some(Some(info)) = unconstrained(rx.recv()).now_or_never() {
        //     app.messages.push_front(info);
        // }

        terminal
            .draw(|frame| {
                let vertical_chunk_constraints = vec![Constraint::Min(1)];

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(vertical_chunk_constraints.as_ref())
                    .split(frame.size());

                let table = Table::new(vec![
                    Row::new(vec!["Row11", "Row12", "Row13"]),
                ])
                    .style(Style::default().fg(Color::White))
                    .header(
                        Row::new(vec!["Col1", "Col2", "Col3"])
                            .style(Style::default().fg(Color::Yellow))
                            .bottom_margin(1)
                    )
                    .block(Block::default().borders(Borders::ALL).title("[ Reddit feed ]"))
                    .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
                    .column_spacing(1)
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

                frame.render_widget(table, vertical_chunks[0])
            })
            .unwrap();

        if let Some(event::Event::Input(input_event)) = &events.next().await {
            if let KeyCode::Esc = input_event.code {
                quitting(terminal);
                break 'outer;
            }
        }
    }

    Ok(())
}