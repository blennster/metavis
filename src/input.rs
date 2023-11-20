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
            KeyCode::Char('j') | KeyCode::Down => {
                move_down(app_state);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                move_up(app_state);
            }
            KeyCode::Char('l') | KeyCode::Right => {
                move_right(app_state);
            }
            KeyCode::Char('h') | KeyCode::Left => {
                move_left(app_state);
            }
            KeyCode::Tab => {
                app_state.focus = app_state.focus.next();
            }
            _ => {}
        }

        if app_state.focus == AppFocus::Source {
            app_state.mark_nodes_under_cursor();
        }
    }

    Ok(())
}

fn move_left(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::Diagnostics => {}
        AppFocus::Source => {
            app_state
                .textarea
                .move_cursor(tui_textarea::CursorMove::Back);
        }
    }
}

fn move_right(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::Diagnostics => {}
        AppFocus::Source => {
            app_state
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
        }
    }
}

fn move_up(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::Diagnostics => {
            app_state.list.up();
        }
        AppFocus::Source => {
            app_state.textarea.move_cursor(tui_textarea::CursorMove::Up);
        }
    }
}

fn move_down(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::Diagnostics => {
            app_state.list.down();
        }
        AppFocus::Source => app_state
            .textarea
            .move_cursor(tui_textarea::CursorMove::Down),
    }
}
