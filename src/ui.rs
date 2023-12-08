use ratatui::{
    prelude::{self, *},
    widgets::{block::Title, *},
    Frame,
};

use crate::app_state;

fn get_border(title: &str, is_in_focus: bool) -> Block<'_> {
    let border = Block::new().borders(Borders::ALL).title(title);
    let green = Style::new().light_green();

    if is_in_focus {
        border.border_type(BorderType::Thick).border_style(green)
    } else {
        border
    }
}

pub fn render(frame: &mut Frame, app_state: &mut app_state::AppState) {
    let area = frame.size();
    let (left_pane, right_upper_pane, right_lower_pane, bottom_pane) = get_layout(&area);

    app_state.sv.update_scroll(&left_pane);
    let source_widget = app_state.sv.get_widget();

    let source_name = &app_state.sv.name;
    frame.render_widget(
        source_widget.block(
            get_border(source_name, app_state.focus == app_state::AppFocus::Source)
                .title(
                    Title::from("[tab]")
                        .alignment(Alignment::Right)
                        .position(block::Position::Bottom),
                )
                .title(Title::from("[s]ource").alignment(Alignment::Right)),
        ),
        left_pane,
    );
    let cursor = app_state.sv.global_cursor(&left_pane);
    frame.set_cursor(cursor.0, cursor.1);

    let widget = app_state.diagnostic_types.widget().block(get_border(
        "[t]ypes",
        app_state.focus == app_state::AppFocus::DiagnosticTypes,
    ));
    frame.render_stateful_widget(
        widget,
        right_upper_pane,
        &mut app_state.diagnostic_types.state,
    );

    let widget = app_state.diagnostics.widget().block(get_border(
        "[d]iagnostics",
        app_state.focus == app_state::AppFocus::Diagnostics,
    ));
    frame.render_stateful_widget(widget, right_lower_pane, &mut app_state.diagnostics.state);

    if app_state.focus == app_state::AppFocus::FilePicker {
        let popup_area = centered_rect(40, 40, area);
        frame.render_widget(Clear, popup_area);
        frame.render_stateful_widget(
            app_state.files.widget().block(get_border(
                "files",
                app_state.focus == app_state::AppFocus::FilePicker,
            )),
            popup_area,
            &mut app_state.files.state,
        );
    }
    if app_state.focus == app_state::AppFocus::LinePicker {
        let popup_area = centered_rect(10, 5, area);
        frame.render_widget(Clear, popup_area);
        frame.render_widget(
            Paragraph::new(app_state.input_buffer.as_str()).block(get_border(
                "goto line:",
                app_state.focus == app_state::AppFocus::LinePicker,
            )),
            popup_area,
        );
    }

    let diags = if app_state.sv.content.is_some() {
        app_state.get_current_diags()
    } else {
        vec![]
    };

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
        .block(get_border("information", false)),
        bottom_pane,
    );
}

fn get_layout(
    area: &prelude::Rect,
) -> (prelude::Rect, prelude::Rect, prelude::Rect, prelude::Rect) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(*area);
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(outer_layout[0]);
    let right_pane = inner_layout[1];
    let right_panes = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(right_pane);

    let right_upper_pane = right_panes[0];
    let right_lower_pane = right_panes[1];
    let left_pane = inner_layout[0];
    let bottom_pane = outer_layout[1];
    (left_pane, right_upper_pane, right_lower_pane, bottom_pane)
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
