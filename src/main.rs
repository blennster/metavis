mod parsers;
mod ui;

use parsers::csv_parser;
use ratatui::{
    prelude::CrosstermBackend,
    widgets::{ListItem, ListState},
    Terminal,
};
use std::{
    io::{self, stdout, BufRead, Result},
    str::FromStr,
};

pub struct App<'a> {
    contents: String,
    nodes: Vec<ListItem<'a>>,
    list_state: ListState,
    should_quit: bool,
    debug_loc: Vec<csv_parser::DebugLoc>,
}

fn main() -> Result<()> {
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let file = std::fs::File::open("./example_data/DEBUG_Loc.csv")?;
    let reader = io::BufReader::new(file);
    let mut debug_loc = Vec::new();
    for line in reader.lines() {
        debug_loc.push(csv_parser::DebugLoc::from_str(&line.unwrap()).unwrap());
    }
    let items = debug_loc.iter().map(|i| ListItem::new(i)).collect();
    let contents = std::fs::read_to_string(format!("./example_data/{}", debug_loc[0].source_file))?;

    let mut app_state = App {
        contents,
        nodes: items,
        list_state,
        should_quit: false,
        debug_loc,
    };

    // Setup terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    // Main loop
    loop {
        ui::main_loop::render(&mut terminal, &mut app_state)?;

        if app_state.should_quit {
            break;
        }
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
