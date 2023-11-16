use ratatui::{
    prelude::{self, *},
    style::Style,
    widgets::*,
    Frame,
};

use crate::{app_state, source_view::SourceView};

pub fn render(frame: &mut Frame, app_state: &mut app_state::AppState) {
    let diag_idx = app_state.diags_state.selected().unwrap_or(0);
    let diag = &app_state.diags[diag_idx];

    let area = frame.size();
    let (left_pane, right_pane, bottom_pane) = get_layout(&area);

    let mut highlights = match app_state.list.selected() {
        Some(d) => d.locs.clone(),
        None => Vec::new(),
    };

    // TODO: This should not be needed but will allow more highlights right now
    highlights.sort_by(|a, b| a.start_line.cmp(&b.start_line));

    let source = SourceView {
        source: diag.source.to_string(),
        highlights,
    };

    let border = Block::new().borders(Borders::ALL);

    match app_state.focus {
        app_state::AppFocus::SOURCE => {
            app_state
                .textarea
                .set_block(border.clone().title(diag.source_file.clone()));
            let widget = app_state.textarea.widget();
            frame.render_widget(widget, left_pane);
        }
        app_state::AppFocus::DIAGNOSTICS => {
            let source_widget = source.get_widget();
            frame.render_widget(
                source_widget.block(border.clone().title(diag.source_file.clone())),
                left_pane,
            );
        }
    }

    let widget = app_state.list.widget().block(border.title("Diagnostics"));
    frame.render_stateful_widget(widget, right_pane, &mut app_state.list.state);
    frame.render_widget(
        Paragraph::new(
            app_state
                .current_nodes
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        ),
        bottom_pane,
    );
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
