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
                return Ok(());
            }
            KeyCode::Char('f') => {
                app_state.focus = AppFocus::FilePicker;
            }
            KeyCode::Esc => {
                if app_state.focus == AppFocus::FilePicker {
                    app_state.focus = AppFocus::Diagnostics;
                }
            }
            _ => {}
        };

        if app_state.focus == AppFocus::Source {
            handle_source_inputs(key, app_state);
            app_state.mark_nodes_under_cursor();
        } else if app_state.focus == AppFocus::Diagnostics {
            handle_diagnostic_inputs(key, app_state);
            let current = &app_state.diagnostics.selected().unwrap().current().unwrap();
            app_state.sv.cursor = (
                (current.start_col - 1) as u16,
                (current.start_line - 1) as u16,
            );
        } else if app_state.focus == AppFocus::FilePicker {
            handle_file_picker_inputs(key, app_state);
        }
    }

    Ok(())
}

fn handle_file_picker_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => app_state.files.down(),
        KeyCode::Char('k') | KeyCode::Up => app_state.files.up(),
        KeyCode::Enter => {
            let file = app_state.files.selected().unwrap().clone();
            app_state.load_file(&file);
            app_state.focus = AppFocus::Diagnostics;
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
        _ => {}
    }
}

fn handle_diagnostic_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.diagnostics.selected().unwrap().unset();
            app_state.diagnostics.down();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.diagnostics.selected().unwrap().unset();
            app_state.diagnostics.up();
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app_state.diagnostics.selected().unwrap().next();
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app_state.diagnostics.selected().unwrap().prev();
        }
        _ => {}
    }
}
