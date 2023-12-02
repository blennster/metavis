use std::rc::Rc;

use crate::{
    list::{self, List},
    parsers::{Diagnostic, MetaInfo, SourceFile},
    source_view::SourceView,
};

#[derive(PartialEq)]
pub enum AppFocus {
    Diagnostics,
    Source,
    FilePicker,
}

impl AppFocus {
    pub fn next(&self) -> AppFocus {
        match self {
            AppFocus::Diagnostics => AppFocus::Source,
            AppFocus::Source => AppFocus::Diagnostics,
            _ => panic!("invalid call to next"),
        }
    }
}

pub struct AppState {
    pub metainfo: MetaInfo,
    pub diagnostics: List<Diagnostic>,
    pub files: List<String>,
    pub should_quit: bool,
    pub focus: AppFocus,
    pub sv: SourceView,
    pub current_nodes: Vec<usize>,
}

impl AppState {
    pub fn new(metainfo: MetaInfo, files: List<String>) -> Self {
        AppState {
            metainfo,
            diagnostics: List::new(vec![]),
            files,
            should_quit: false,
            focus: AppFocus::FilePicker,
            sv: SourceView::new(),
            current_nodes: vec![],
        }
    }

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
        let (col, row) = self.sv.cursor;
        self.current_nodes = self.nodes_at(row.into(), col.into());
        self.diagnostics
            .mark(|d| d.nodes.iter().any(|n| self.current_nodes.contains(n)));
    }

    pub fn get_current_diags(&self) -> Vec<&Diagnostic> {
        let diags = self
            .diagnostics
            .items
            .iter()
            .filter(|d| d.nodes.iter().any(|n| self.current_nodes.contains(n)))
            .collect::<Vec<_>>();

        diags
    }

    /// Load a file from the project
    pub fn load_file(&mut self, file: &str) {
        let diags = self.metainfo.get_diags_for_file(file);
        let mut l = vec![];
        for d in &diags {
            l.push(d.clone());
        }
        let diagnostics = list::List::new(l);
        self.diagnostics = diagnostics;

        let mut sv = SourceView::new();
        sv.content = diags[0].source.content.clone();
        sv.name = diags[0].source.name.clone();

        let diag = self.diagnostics.selected().unwrap();
        diag.set();
        let highlights = diag.locs.clone();
        sv.highlights = highlights;

        self.sv = sv;
    }
}
