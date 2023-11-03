mod parser;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Stylize, Terminal},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{
    cell::Cell,
    cmp,
    io::{stdout, Result},
    rc::Rc,
};

fn main() -> Result<()> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    // Main loop
    loop {
        let items = vec![ListItem::new("Item 1"), ListItem::new("Item 2")];
        let items_len = items.len();

        terminal.draw(|frame| {
            let area = frame.size();
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(area);
            let inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(outer_layout[0]);

            frame.render_widget(Block::new().borders(Borders::ALL), inner_layout[0]);
            frame.render_widget(Block::new().borders(Borders::ALL), inner_layout[1]);
            frame.render_stateful_widget(
                List::new(items)
                    .block(Block::new().borders(Borders::ALL).title("List"))
                    .highlight_symbol(">>"),
                outer_layout[1],
                &mut list_state,
            )
        })?;

        if event::poll(std::time::Duration::from_millis(5000))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Char('j') => {
                            list_state.select(Some(cmp::min(
                                items_len,
                                list_state.selected().unwrap_or(0) + 1,
                            )));
                        }
                        KeyCode::Char('k') => {
                            list_state
                                .select(Some(cmp::max(0, list_state.selected().unwrap_or(0) - 1)));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
