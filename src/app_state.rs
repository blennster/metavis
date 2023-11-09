use ratatui::widgets::{ListItem, ListState};

use crate::parsers::loc_file::DebugLoc;

pub struct AppState<'a> {
    pub source: String,
    pub nodes: Vec<ListItem<'a>>,
    pub list_state: ListState,
    pub should_quit: bool,
    pub debug_locs: Vec<DebugLoc>,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            nodes: Vec::new(),
            list_state: ListState::default(),
            should_quit: false,
            debug_locs: Vec::new(),
        }
    }
}
