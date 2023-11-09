use std::rc::Rc;

use ratatui::{
    prelude::{self, Constraint, Direction, Layout, Rect, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::source_view::SourceView;

pub fn render(frame: &mut Frame, app_state: &mut crate::AppState) {
    let items = app_state.nodes.clone();

    let area = frame.size();
    let (outer_layout, inner_layout) = get_layout(&area);
    let left_pane = inner_layout[0];
    let right_pane = inner_layout[1];
    let bottom_pane = outer_layout[1];

    let source = SourceView {
        name: String::from("Some source"),
        source: app_state.source.clone(),
        highlights: app_state
            .debug_locs
            .iter()
            .map(|dl| dl.loc.clone())
            .collect(),
    };

    source.render(frame, left_pane);

    // frame.render_widget(
    //     Paragraph::new(app_state.source.clone())
    //         .block(Block::new().borders(Borders::ALL).title("Source")),
    //     left_pane,
    // );
    frame.render_stateful_widget(
        List::new(items)
            .block(Block::new().borders(Borders::ALL).title("Nodes"))
            .highlight_symbol(">>"),
        right_pane,
        &mut app_state.list_state,
    );
    frame.render_widget(Block::new().borders(Borders::ALL), bottom_pane);
}

fn get_layout(area: &prelude::Rect) -> (Rc<[prelude::Rect]>, Rc<[prelude::Rect]>) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(*area);
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(outer_layout[0]);
    (outer_layout, inner_layout)
}
