mod app_state;
mod file_picker;
mod input;
mod list;
mod parsers;
mod source_view;
mod ui;

use ratatui::{prelude::CrosstermBackend, Terminal};
use source_view::SourceView;
use std::io::stdout;

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

fn make_app_state<'a>(source_dir: &str) -> anyhow::Result<app_state::AppState> {
    let metainfo = parsers::MetaInfo::new(source_dir);

    let files = metainfo.source_files.keys().cloned().collect();
    let files = list::List::new(files);

    let app_state = app_state::AppState::new(metainfo, files);
    // let app_state = app_state::AppState {
    //     focus: app_state::AppFocus::Diagnostics,
    //     source: diags[0].source.clone(),
    //     metainfo,
    //     sv,
    //     current_nodes: vec![],
    //     should_quit: false,
    //     diagnostics: diaglist,
    //     files: list::List::new(vec![]),
    // };

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
