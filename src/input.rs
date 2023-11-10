use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::cmp;

use crate::app_state::AppFocus;

pub fn handle_events(app_state: &mut crate::app_state::AppState) -> std::io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        let focus = &mut app_state.focus;
        let (focused_len, focused_state) = match focus {
            AppFocus::DIAGNOSTICS => (app_state.diags.len(), &mut app_state.diags_state),
        };

        match key.code {
            KeyCode::Char('q') => {
                app_state.should_quit = true;
                return Ok(());
            }
            KeyCode::Char('j') => {
                focused_state.select(Some(cmp::min(
                    focused_len - 1,
                    focused_state.selected().unwrap_or(0) + 1,
                )));
            }
            KeyCode::Char('k') => {
                focused_state.select(match focused_state.selected() {
                    Some(0) => Some(0),
                    Some(x) => Some(x - 1),
                    None => Some(0),
                });
            }
            // KeyCode::Char('J') => {
            //     if *focus == AppFocus::NODES {
            //         *focus = AppFocus::DIAGNOSTICS;
            //     }
            // }
            // KeyCode::Char('K') => {
            //     if *focus == AppFocus::DIAGNOSTICS {
            //         *focus = AppFocus::NODES;
            //     }
            // }
            _ => {}
        }
    }

    Ok(())
}
