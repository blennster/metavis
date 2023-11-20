use std::rc::Rc;

use ratatui::widgets::ListState;

use crate::{
    list::List,
    parsers::{Diagnostic, MetaInfo, SourceFile},
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
