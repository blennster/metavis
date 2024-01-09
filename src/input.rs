use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app_state::AppFocus;

pub fn handle_events(app_state: &mut crate::app_state::AppState) -> std::io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        if key.modifiers.contains(event::KeyModifiers::CONTROL) && key.code == KeyCode::Char('c')
            || key.code == KeyCode::Char('q')
        {
            app_state.should_quit = true;
            return Ok(());
        }

        match key.code {
            KeyCode::Tab => {
                app_state.focus = app_state.focus.next();
            }
            KeyCode::BackTab => {
                app_state.focus = app_state.focus.prev();
            }
            KeyCode::Char('r') => {
                app_state.focus = AppFocus::Relations;
            }
            KeyCode::Char('t') => {
                app_state.focus = AppFocus::Tuples;
            }
            KeyCode::Char('s') => {
                app_state.focus = AppFocus::Source;
            }
            KeyCode::Char('f') => {
                if !app_state.files.items.is_empty() {
                    app_state.focus = AppFocus::FilePicker;
                }
            }
            KeyCode::Esc => {
                if app_state.focus == AppFocus::FilePicker {
                    app_state.focus = AppFocus::Source;
                } else if app_state.focus == AppFocus::LinePicker {
                    app_state.focus = AppFocus::Source;
                } else {
                    app_state.should_quit = true;
                }
            }
            _ => {}
        };

        if app_state.focus == AppFocus::Source {
            handle_source_inputs(key, app_state);
            app_state.mark_nodes_under_cursor();
        } else if app_state.focus == AppFocus::Relations {
            handle_relations_inputs(key, app_state);
            app_state.update_view();
            if let Some(s) = app_state.tuples.selected() {
                s.set()
            }
        } else if app_state.focus == AppFocus::Tuples {
            // Prevent crashes by returning early
            if app_state.tuples.items.is_empty() {
                return Ok(());
            }
            handle_tuples_inputs(key, app_state);
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
        KeyCode::Backspace => {
            app_state.input_buffer.pop();
        }
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

fn handle_relations_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.relations.down();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.relations.up();
        }
        KeyCode::Enter => {
            if let Some(s) = app_state.relations.selected() {
                s.unmark();
            }
            app_state.relations.confirm();
            if let Some(s) = app_state.relations.selected() {
                s.mark();
            }
            let category = app_state.relations.selected().unwrap().name.clone();
            app_state.get_tuples_for_relation(&category);
        }
        _ => {}
    }
}

fn handle_file_picker_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => app_state.files.down(),
        KeyCode::Char('k') | KeyCode::Up => app_state.files.up(),
        KeyCode::Enter => {
            app_state.files.confirm();
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

fn handle_tuples_inputs(key: event::KeyEvent, app_state: &mut crate::app_state::AppState) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.tuples.down();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.tuples.up();
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if let Some(s) = app_state.tuples.selected() {
                s.next()
            }
        }
        KeyCode::Char('h') | KeyCode::Left => {
            if let Some(s) = app_state.tuples.selected() {
                s.prev()
            }
        }
        KeyCode::Enter => {
            if let Some(s) = app_state.tuples.selected() {
                s.unset()
            }
            app_state.tuples.confirm();
            if let Some(s) = app_state.tuples.selected() {
                s.set()
            }
        }
        _ => {}
    }
}
