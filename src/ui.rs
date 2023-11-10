use ratatui::{
    prelude::{self, Constraint, Direction, Layout, Rect, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{app_state, source_view::SourceView};

pub fn render(frame: &mut Frame, app_state: &mut app_state::AppState) {
    let diag_idx = app_state.diags_state.selected().unwrap_or(0);
    let diag = &app_state.diags[diag_idx];

    // TODO: move this into state to not collect every render
    let diags: Vec<ListItem> = app_state
        .diags
        .iter()
        .map(|d| Into::<ListItem>::into(d))
        .collect();

    let area = frame.size();
    let (left_pane, right_pane, bottom_pane) = get_layout(&area);

    let mut highlights = diag.locs.clone();

    // TODO: This should not be needed but will allow more highlights right now
    highlights.sort_by(|a, b| a.start_line.cmp(&b.start_line));

    let source = SourceView {
        source: diag.source.to_string(),
        highlights,
    };

    let border = Block::new().borders(Borders::ALL);

    let diags_widget = List::new(diags)
        .block(border.clone().title("Diagnostics"))
        .highlight_symbol(">>");

    // match app_state.focus {
    //     app_state::AppFocus::DIAGNOSTICS => diags_widget = diags_widget.slow_blink(),
    // };

    let source_widget = source.get_widget();

    frame.render_widget(
        source_widget.block(border.clone().title(diag.source_file.clone())),
        left_pane,
    );
    frame.render_stateful_widget(diags_widget, right_pane, &mut app_state.diags_state);
    frame.render_widget(border, bottom_pane);
}

fn get_layout(area: &prelude::Rect) -> (prelude::Rect, prelude::Rect, prelude::Rect) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(*area);
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(outer_layout[0]);
    let left_pane = inner_layout[0];
    let right_pane = inner_layout[1];
    let bottom_pane = outer_layout[1];
    (left_pane, right_pane, bottom_pane)
}
