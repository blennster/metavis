use std::rc::Rc;

use ratatui::widgets::ListState;

use crate::{
    list::List,
    parsers::{Diagnostic, MetaInfo, SourceFile},
};

#[derive(PartialEq)]
pub enum AppFocus {
    Diagnostics,
    Source,
}

impl AppFocus {
    pub fn next(&self) -> AppFocus {
        match self {
            AppFocus::Diagnostics => AppFocus::Source,
            AppFocus::Source => AppFocus::Diagnostics,
        }
    }
}

pub struct AppState<'a> {
    pub source: Rc<SourceFile>,
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
        // Adjust for indexing
        let row = row + 1;
        let col = col + 1;

        self.metainfo
            .debug_locs
            .iter()
            .filter(|d| {
                d.loc.start_line <= row
                    && d.loc.start_col <= col
                    && d.loc.end_col >= col
                    && d.loc.end_line >= row
            })
            .map(|d| d.node_id)
            .collect()
    }

    pub fn mark_nodes_under_cursor(&mut self) {
        let (row, col) = self.textarea.cursor();
        self.current_nodes = self.nodes_at(row, col);
        self.list
            .mark(|d| d.nodes.iter().any(|n| self.current_nodes.contains(n)));
    }

    pub fn get_current_diags(&self) -> Vec<&Diagnostic> {
        let diags = self
            .diags
            .iter()
            .filter(|d| d.nodes.iter().any(|n| self.current_nodes.contains(n)))
            .collect::<Vec<_>>();

        diags
    }
}
