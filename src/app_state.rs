use std::rc::Rc;

use ratatui::widgets::ListState;

use crate::parsers::{Diagnostic, MetaInfo};

#[derive(PartialEq)]
pub enum AppFocus {
    DIAGNOSTICS,
}

pub struct AppState {
    pub source: Rc<str>,
    pub metainfo: MetaInfo,
    pub diags: Vec<Diagnostic>,
    pub diags_state: ListState,
    pub should_quit: bool,
    pub focus: AppFocus,
}

impl AppState {}
