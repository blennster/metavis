use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app_state::AppFocus;

pub fn handle_events(app_state: &mut crate::app_state::AppState) -> std::io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') => {
                app_state.should_quit = true;
                return Ok(());
            }
            KeyCode::Tab => {
                app_state.focus = app_state.focus.next();
            }
            KeyCode::Char('t') => {
                if app_state.focus == AppFocus::DiagnosticTypes {
                    app_state.focus = app_state.focus.next();
                } else {
                    app_state.focus = AppFocus::DiagnosticTypes;
                }
            }
            KeyCode::Char('d') => {
                if app_state.focus == AppFocus::Diagnostics {
                    app_state.focus = app_state.focus.next();
                } else {
                    app_state.focus = AppFocus::Diagnostics;
                }
            }
            KeyCode::Char('s') => {
                if app_state.focus == AppFocus::Source {
                    app_state.focus = app_state.focus.next();
                } else {
                    app_state.focus = AppFocus::Source;
                }
            }
            KeyCode::Char('f') => {
                if !app_state.files.items.is_empty() {
                    app_state.focus = AppFocus::FilePicker;
                }
            }
            KeyCode::Esc => {
                if app_state.focus == AppFocus::FilePicker {
                    app_state.focus = AppFocus::Source;
                }
                if app_state.focus == AppFocus::LinePicker {
                    app_state.focus = AppFocus::Source;
                }
            }
            _ => {}
        };

        if app_state.focus == AppFocus::Source {
            handle_source_inputs(key, app_state);
            app_state.mark_nodes_under_cursor();
        } else if app_state.focus == AppFocus::DiagnosticTypes {
            handle_diagnostic_type_inputs(key, app_state);
            app_state.update_view();
            if let Some(s) = app_state.diagnostics.selected() {
                s.set()
            }
        } else if app_state.focus == AppFocus::Diagnostics {
            // Prevent crashes by returning early
            if app_state.diagnostics.items.is_empty() {
                return Ok(());
            }
            handle_diagnostic_inputs(key, app_state);
            app_state.update_view();
        } else if app_state.focus == AppFocus::FilePicker {
            handle_file_picker_inputs(key, app_state);
        } else if app_state.focus == AppFocus::LinePicker {
            handle_line_picker_inputs(key, app_state);
        }
    }

    Ok(())
}

fn handle_line_picker_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('0') => app_state.input_buffer.push_str("0"),
        KeyCode::Char('1') => app_state.input_buffer.push_str("1"),
        KeyCode::Char('2') => app_state.input_buffer.push_str("2"),
        KeyCode::Char('3') => app_state.input_buffer.push_str("3"),
        KeyCode::Char('4') => app_state.input_buffer.push_str("4"),
        KeyCode::Char('5') => app_state.input_buffer.push_str("5"),
        KeyCode::Char('6') => app_state.input_buffer.push_str("6"),
        KeyCode::Char('7') => app_state.input_buffer.push_str("7"),
        KeyCode::Char('8') => app_state.input_buffer.push_str("8"),
        KeyCode::Char('9') => app_state.input_buffer.push_str("9"),
        KeyCode::Enter => {
            let target = app_state.input_buffer.parse::<u16>();
            if let Ok(target) = target {
                app_state.sv.move_to((0, target - 1));
            }
            app_state.focus = AppFocus::Source;
        }
        _ => {}
    };
}

fn handle_diagnostic_type_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.diagnostic_types.down();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.diagnostic_types.up();
        }
        _ => {}
    }
    let category = &app_state.diagnostic_types.selected().unwrap().clone();
    app_state.get_diags_for_category(category);
}

fn handle_file_picker_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => app_state.files.down(),
        KeyCode::Char('k') | KeyCode::Up => app_state.files.up(),
        KeyCode::Enter => {
            let file = app_state.files.selected().unwrap().clone();
            app_state.load_file(&file);
            app_state.focus = AppFocus::Source;
        }
        _ => {}
    }
}

fn handle_source_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state
                .sv
                .move_cursor(crate::source_view::Direction::Down);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.sv.move_cursor(crate::source_view::Direction::Up);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app_state
                .sv
                .move_cursor(crate::source_view::Direction::Right);
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app_state
                .sv
                .move_cursor(crate::source_view::Direction::Left);
        }
        KeyCode::Char('g') | KeyCode::Home => {
            app_state.sv.move_to_start();
        }
        KeyCode::Char('G') | KeyCode::End => {
            app_state.sv.move_to_end();
        }
        KeyCode::Char(':') => {
            app_state.focus = AppFocus::LinePicker;
            app_state.input_buffer.clear();
        }
        _ => {}
    }
}

fn handle_diagnostic_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            if let Some(s) = app_state.diagnostics.selected() {
                s.unset()
            }
            app_state.diagnostics.down();
            if let Some(s) = app_state.diagnostics.selected() {
                s.set()
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if let Some(s) = app_state.diagnostics.selected() {
                s.unset()
            }
            app_state.diagnostics.up();
            if let Some(s) = app_state.diagnostics.selected() {
                s.set()
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if let Some(s) = app_state.diagnostics.selected() {
                s.next()
            }
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if let Some(s) = app_state.diagnostics.selected() {
                s.prev()
            }
        }
        _ => {}
    }
}
