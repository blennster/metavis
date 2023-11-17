use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::cmp;

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

        let (row, col) = app_state.textarea.cursor();
        app_state.current_nodes = app_state.nodes_at(row, col);
        app_state
            .list
            .mark(|d| d.nodes.iter().any(|n| app_state.current_nodes.contains(n)));
    }

    Ok(())
}

fn move_left(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::DIAGNOSTICS => {}
        AppFocus::SOURCE => {
            app_state
                .textarea
                .move_cursor(tui_textarea::CursorMove::Back);
        }
    }
}

fn move_right(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::DIAGNOSTICS => {}
        AppFocus::SOURCE => {
            app_state
                .textarea
                .move_cursor(tui_textarea::CursorMove::Forward);
        }
    }
}

fn move_up(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::DIAGNOSTICS => {
            app_state.list.up();
            // let diags = &mut app_state.diags_state;
            // diags.select(match diags.selected() {
            //     Some(0) => Some(0),
            //     Some(x) => Some(x - 1),
            //     None => Some(0),
            // });
        }
        AppFocus::SOURCE => {
            app_state.textarea.move_cursor(tui_textarea::CursorMove::Up);
        }
    }
}

fn move_down(app_state: &mut crate::app_state::AppState) {
    match app_state.focus {
        AppFocus::DIAGNOSTICS => {
            app_state.list.down();
            // let diags = &mut app_state.diags_state;
            // let len = app_state.diags.len();
            // diags.select(Some(cmp::min(len - 1, diags.selected().unwrap_or(0) + 1)));
        }
        AppFocus::SOURCE => app_state
            .textarea
            .move_cursor(tui_textarea::CursorMove::Down),
    }
}
