use ratatui::{
    prelude::{self, *},
    widgets::*,
    Frame,
};

use crate::{app_state, source_view::SourceView};

pub fn render(frame: &mut Frame, app_state: &mut app_state::AppState) {
    let area = frame.size();
    let (left_pane, right_pane, bottom_pane) = get_layout(&area);

    let border = Block::new().borders(Borders::ALL);

    app_state.sv.update_scroll(&left_pane);
    let source_widget = app_state.sv.get_widget();
    let source_name = app_state.sv.name.clone();
    frame.render_widget(
        source_widget.block(border.clone().title(source_name)),
        left_pane,
    );
    let cursor = app_state.sv.global_cursor(&left_pane);
    frame.set_cursor(cursor.0, cursor.1);

    let widget = app_state
        .diagnostics
        .widget()
        .block(border.clone().title("Diagnostics"));
    frame.render_stateful_widget(widget, right_pane, &mut app_state.diagnostics.state);

    if app_state.focus == app_state::AppFocus::FilePicker {
        let popup_area = centered_rect(40, 40, area);
        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            app_state
                .files
                .widget()
                .block(border.clone().title("Files")),
            popup_area,
            &mut app_state.files.state,
        );
    }

    let diags = app_state.get_current_diags();
    frame.render_widget(
        Paragraph::new(
            diags
                .iter()
                .map(|d| {
                    format!(
                        "{}: ({})",
                        d.name,
                        d.nodes
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        )
        .wrap(Wrap { trim: false })
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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
