mod app_state;
mod input;
mod list;
mod parsers;
mod source_view;
mod ui;

use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::stdout;

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

fn make_app_state(source_dir: &str) -> anyhow::Result<app_state::AppState> {
    let metainfo = parsers::MetaInfo::new(source_dir);

    let mut files = metainfo.source_files.keys().cloned().collect::<Vec<_>>();
    files.sort();
    let files = list::List::new(files);

    let mut app_state = app_state::AppState::new(metainfo, files);
    let category = app_state.diagnostic_types.items[0].clone();
    app_state.get_diags_for_category(&category);
    app_state.update_view();
    if let Some(s) = app_state.diagnostics.selected() { s.set() }

    Ok(app_state)
}

fn main() -> anyhow::Result<()> {
    coredump::register_panic_handler().unwrap();
    initialize_panic_handler();

    let mut args = std::env::args();
    args.next();
    let root = match args.next() {
        Some(root) => root,
        None => {
            // Panic if not in debug, otherwise fallback to example_data
            #[cfg(not(debug_assertions))]
            panic!("Please specify the root directory of the project");
            "./example_data".to_string()
        }
    };

    // Initalize app_state before chaning the terminal
    let mut app_state = make_app_state(&root)?;

    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    crossterm::execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;

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
