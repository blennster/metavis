mod app_state;
mod input;
mod parsers;
mod source_view;
mod ui;

use parsers::loc_file::DebugLoc;

use ratatui::{
    prelude::CrosstermBackend,
    widgets::{ListItem, ListState},
    Terminal,
};
use std::{
    io::{self, stdout, BufRead, Result},
    str::FromStr,
};

type AppState<'a> = app_state::AppState<'a>;

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

fn main() -> Result<()> {
    let _ = coredump::register_panic_handler();
    initialize_panic_handler();
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let file = std::fs::File::open("./example_data/DEBUG_Loc.csv")?;
    let reader = io::BufReader::new(file);
    let mut debug_locs = Vec::new();
    for line in reader.lines() {
        debug_locs.push(DebugLoc::from_str(&line.unwrap()).unwrap());
    }
    let items = debug_locs.iter().map(|i| ListItem::new(i)).collect();
    let contents =
        std::fs::read_to_string(format!("./example_data/{}", debug_locs[0].source_file))?;

    let mut app_state = AppState {
        source: contents,
        nodes: items,
        list_state,
        should_quit: false,
        debug_locs,
    };

    // Setup terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // Main loop
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app_state))?;
        input::handle_events(&mut app_state)?;

        if app_state.should_quit {
            break;
        }
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
