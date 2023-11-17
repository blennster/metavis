mod app_state;
mod input;
mod list;
mod parsers;
mod source_view;
mod ui;

use ratatui::{
    prelude::CrosstermBackend,
    widgets::{ListItem, ListState},
    Terminal,
};
use std::{
    io::{self, stdout, BufRead, Result},
    str::FromStr,
};

pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

fn main() -> Result<()> {
    coredump::register_panic_handler().unwrap();
    initialize_panic_handler();

    let metainfo = parsers::MetaInfo::new("./example_data");
    let diags = metainfo.get_diags_for_file("tests/clang/evaluation/src/arena/test1.c");
    let mut textarea = tui_textarea::TextArea::from(diags[0].source.to_owned().split('\n'));
    textarea.set_line_number_style(ratatui::style::Style::default());
    let mut l = vec![];
    for d in &diags {
        l.push(d.clone());
    }
    let diaglist = list::List::new(l);

    let mut app_state = app_state::AppState {
        focus: app_state::AppFocus::DIAGNOSTICS,
        source_name: diags[0].source_file.clone().into(),
        source: diags[0].source.clone(),
        textarea,
        diags,
        diags_state: ListState::default().with_selected(Some(0)),
        metainfo,
        current_nodes: vec![],
        should_quit: false,
        list: diaglist,
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
