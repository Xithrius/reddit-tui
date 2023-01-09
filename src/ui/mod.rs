use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::handlers::{app::App, config::CompleteConfig};

pub fn draw_ui<T: Backend>(frame: &mut Frame<T>, _app: &mut App, _config: &CompleteConfig) {
    let vertical_chunk_constraints = vec![Constraint::Min(1)];

    let _vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vertical_chunk_constraints.as_ref())
        .split(frame.size());

    // let table = Table::new(
    //     posts
    //         .iter()
    //         .map(|f| Row::new(vec![f.title.as_str()]))
    //         .collect::<Vec<Row>>(),
    // )
    // .style(Style::default().fg(Color::White))

    // .widths(&[Constraint::Percentage(100)])
    // .column_spacing(1)
    // .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    // let items = table_events
    //     .posts
    //     .iter()
    //     .map(|i| Row::new(vec![i.as_ref()]))
    //     .collect::<Vec<Row>>();

    let table = Table::new(vec![])
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Title"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("[ Reddit feed ]"),
        );

    frame.render_widget(table, frame.size());
}
