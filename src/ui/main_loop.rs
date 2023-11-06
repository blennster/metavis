use std::{cmp, io::Stdout, rc::Rc};

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    prelude::{self, Constraint, CrosstermBackend, Direction, Layout, Rect, Stylize, Terminal},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn render(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut crate::App,
) -> std::io::Result<()> {
    let items = app.nodes.clone();
    let items_len = items.len();

    terminal.draw(|frame| {
        let area = frame.size();
        let (outer_layout, inner_layout) = get_layout(&area);

        frame.render_widget(
            Paragraph::new(app.contents.clone())
                .block(Block::new().borders(Borders::ALL).title("Source")),
            inner_layout[0],
        );
        frame.render_stateful_widget(
            List::new(items)
                .block(Block::new().borders(Borders::ALL).title("Nodes"))
                .highlight_symbol(">>"),
            inner_layout[1],
            &mut app.list_state,
        );
        frame.render_widget(Block::new().borders(Borders::ALL), outer_layout[1]);
    })?;

    if event::poll(std::time::Duration::from_millis(5000))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                        return Ok(());
                    }
                    KeyCode::Char('j') => {
                        app.list_state.select(Some(cmp::min(
                            items_len - 1,
                            app.list_state.selected().unwrap_or(0) + 1,
                        )));
                    }
                    KeyCode::Char('k') => {
                        app.list_state.select(match app.list_state.selected() {
                            Some(0) => Some(0),
                            Some(x) => Some(x - 1),
                            None => Some(0),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn get_layout(area: &prelude::Rect) -> (Rc<[prelude::Rect]>, Rc<[prelude::Rect]>) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(*area);
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(outer_layout[0]);
    (outer_layout, inner_layout)
}
