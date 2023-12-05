use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{app_state::AppFocus, source_view::SourceView};

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
            KeyCode::Char('t') => {
                if app_state.focus == AppFocus::DiagnosticTypes {
                    app_state.focus = app_state.focus.next();
                } else {
                    app_state.focus = AppFocus::DiagnosticTypes;
                }
                return Ok(());
            }
            KeyCode::Char('d') => {
                if app_state.focus == AppFocus::Diagnostics {
                    app_state.focus = app_state.focus.next();
                } else {
                    app_state.focus = AppFocus::Diagnostics;
                }
                return Ok(());
            }
            KeyCode::Char('s') => {
                if app_state.focus == AppFocus::Source {
                    app_state.focus = app_state.focus.next();
                } else {
                    app_state.focus = AppFocus::Source;
                }
                return Ok(());
            }
            KeyCode::Char('f') => {
                app_state.focus = AppFocus::FilePicker;
                return Ok(());
            }
            KeyCode::Esc => {
                if app_state.focus == AppFocus::FilePicker {
                    app_state.focus = AppFocus::Source;
                }
                return Ok(());
            }
            _ => {}
        };

        if app_state.focus == AppFocus::Source {
            handle_source_inputs(key, app_state);
            app_state.mark_nodes_under_cursor();
        } else if app_state.focus == AppFocus::DiagnosticTypes {
            handle_diagnostic_type_inputs(key, app_state);
        } else if app_state.focus == AppFocus::Diagnostics {
            // Prevent crashes by returning early
            if app_state.diagnostics.items.len() == 0 {
                return Ok(());
            }

            handle_diagnostic_inputs(key, app_state);
            let selected = &app_state.diagnostics.selected().unwrap();
            app_state.sv.highlights = selected.locs.clone();
            let current = selected.current().unwrap();
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
        _ => {}
    }
}

fn handle_diagnostic_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.diagnostics.selected().unwrap().unset();
            app_state.diagnostics.down();
            app_state.diagnostics.selected().unwrap().set();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.diagnostics.selected().unwrap().unset();
            app_state.diagnostics.up();
            app_state.diagnostics.selected().unwrap().set();
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app_state.diagnostics.selected().unwrap().next();
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app_state.diagnostics.selected().unwrap().prev();
        }
        _ => {}
    }

    let diag = app_state.diagnostics.selected().unwrap();
    let loc = diag.current().unwrap();

    if app_state.sv.name != loc.source_file {
        let mut sv = SourceView::new();
        sv.name = loc.source_file.clone();
        sv.content = match app_state.metainfo.source_files.get(&sv.name) {
            Some(s) => s.content.clone(),
            None => "-- FILE NOT FOUND --".to_owned(),
        };
        app_state.sv = sv;
    }

    let sv = &mut app_state.sv;
    sv.cursor = ((loc.start_col - 1) as u16, (loc.start_line - 1) as u16);
    sv.highlights = diag.locs.clone();
}
