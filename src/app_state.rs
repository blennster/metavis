use std::rc::Rc;

use ratatui::widgets::ListState;

use crate::{
    diagnostic_list::List,
    parsers::{Diagnostic, MetaInfo},
};

#[derive(PartialEq)]
pub enum AppFocus {
    DIAGNOSTICS,
    SOURCE,
}

impl AppFocus {
    pub fn next(&self) -> AppFocus {
        match self {
            AppFocus::DIAGNOSTICS => AppFocus::SOURCE,
            AppFocus::SOURCE => AppFocus::DIAGNOSTICS,
        }
    }
}

pub struct AppState<'a> {
    pub source_name: Rc<str>,
    pub source: Rc<str>,
    pub metainfo: MetaInfo,
    pub diags: Vec<Diagnostic>,
    pub diags_state: ListState,
    pub list: List<Diagnostic>,
    pub should_quit: bool,
    pub focus: AppFocus,
    pub textarea: tui_textarea::TextArea<'a>,
    pub current_nodes: Vec<usize>,
}

impl<'a> AppState<'a> {
    pub fn nodes_at(&self, row: usize, col: usize) -> Vec<usize> {
        self.metainfo
            .debug_locs
            .iter()
            .filter(|d| {
                d.loc.start_line <= row + 1
                    && d.loc.start_col <= col + 1
                    && d.loc.end_col >= col + 1
                    && d.loc.end_line >= row + 1
            })
            .map(|d| d.node_id)
            .collect()
    }
}
