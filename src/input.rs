use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::cmp;

pub fn handle_events(app_state: &mut crate::AppState) -> std::io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') => {
                app_state.should_quit = true;
                return Ok(());
            }
            KeyCode::Char('j') => {
                let items_len = app_state.nodes.len();
                app_state.list_state.select(Some(cmp::min(
                    items_len - 1,
                    app_state.list_state.selected().unwrap_or(0) + 1,
                )));
            }
            KeyCode::Char('k') => {
                app_state
                    .list_state
                    .select(match app_state.list_state.selected() {
                        Some(0) => Some(0),
                        Some(x) => Some(x - 1),
                        None => Some(0),
                    });
            }
            _ => {}
        }
    }

    Ok(())
}
