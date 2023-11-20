use ratatui::{
    prelude::{self, *},
    widgets::*,
    Frame,
};

use crate::{app_state, source_view::SourceView};

pub fn render(frame: &mut Frame, app_state: &mut app_state::AppState) {
    let diag = app_state.list.selected().unwrap();

    let area = frame.size();
    let (left_pane, right_pane, bottom_pane) = get_layout(&area);

    let highlights = match app_state.list.selected() {
        Some(d) => d.locs.clone(),
        None => Vec::new(),
    };

    let source_name = diag.source.name.clone();
    let source_content = diag.source.content.clone();

    let source = SourceView {
        content: source_content,
        highlights,
    };

    let border = Block::new().borders(Borders::ALL);

    match app_state.focus {
        app_state::AppFocus::Source => {
            app_state
                .textarea
                .set_block(border.clone().title(source_name));
            let widget = app_state.textarea.widget();
            frame.render_widget(widget, left_pane);
        }
        app_state::AppFocus::Diagnostics => {
            let source_widget = source.get_widget();
            frame.render_widget(
                source_widget.block(border.clone().title(source_name)),
                left_pane,
            );
        }
    }

    let widget = app_state
        .list
        .widget()
        .block(border.clone().title("Diagnostics"));
    frame.render_stateful_widget(widget, right_pane, &mut app_state.list.state);

    let diags = app_state.get_current_diags();
    frame.render_widget(
        Paragraph::new(
            diags
                .iter()
                .map(|d| d.name.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
        .block(border.title("Diagnostics")),
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
