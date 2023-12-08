use crate::{
    list::{self, List},
    parsers::{Diagnostic, MetaInfo},
    source_view::SourceView,
};

#[derive(PartialEq)]
pub enum AppFocus {
    DiagnosticTypes,
    Diagnostics,
    Source,
    FilePicker,
}

impl AppFocus {
    pub fn next(&self) -> AppFocus {
        match self {
            AppFocus::Diagnostics => AppFocus::Source,
            AppFocus::Source => AppFocus::Diagnostics,
            AppFocus::DiagnosticTypes => AppFocus::Diagnostics,
            _ => panic!("invalid call to next"),
        }
    }
}

pub struct AppState {
    pub metainfo: MetaInfo,
    pub diagnostics: List<Diagnostic>,
    pub files: List<String>,
    pub diagnostic_types: List<String>,
    pub should_quit: bool,
    pub focus: AppFocus,
    pub sv: SourceView,
    pub current_nodes: Vec<usize>,
}

impl AppState {
    pub fn new(metainfo: MetaInfo, files: List<String>) -> Self {
        let mut diagnostic_types = metainfo
            .diagnostics
            .iter()
            .map(|d| d.name.clone())
            .collect::<Vec<_>>();
        diagnostic_types.dedup();

        AppState {
            metainfo,
            diagnostic_types: List::new(diagnostic_types),
            diagnostics: List::new(vec![]),
            files,
            should_quit: false,
            focus: AppFocus::DiagnosticTypes,
            sv: SourceView::new(),
            current_nodes: vec![],
        }
    }

    pub fn nodes_at(&self, row: usize, col: usize) -> Vec<usize> {
        // Adjust for indexing
        let row = row + 1;

        self.metainfo
            .debug_locs
            .iter()
            .filter(|d| {
                let h = &d.loc;
                if d.loc.source_file == self.sv.name && (h.start_line <= row && row <= h.end_line) {
                    if h.start_line == h.end_line {
                        h.start_col <= col && col <= h.end_col
                    } else if h.start_line == row {
                        h.start_col <= col
                    } else if h.end_line == row {
                        col <= h.end_col
                    } else {
                        true
                    }
                } else {
                    false
                }
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

    pub fn get_current_diags(&self) -> Vec<Diagnostic> {
        let diags = self.metainfo.get_diags(&self.current_nodes);

        diags
    }

    pub fn get_diags_for_category(&mut self, category: &str) {
        let diags = self.metainfo.get_diags_for_category(category);
        let mut l = vec![];
        for d in &diags {
            l.push(d.clone());
        }
        let diagnostics = list::List::new(l);
        self.diagnostics = diagnostics;
    }

    /// Load a file from the project
    pub fn load_file(&mut self, file: &str) {
        let sc = &self.metainfo.source_files[file];

        let mut sv = SourceView::new();
        sv.content = Some(sc.content.clone());
        sv.name = file.to_owned();

        self.sv = sv;
    }

    pub fn update_view(&mut self) {
        let diag = self.diagnostics.selected().unwrap();
        let loc = diag.current().unwrap();

        if self.sv.name != loc.source_file {
            let mut sv = SourceView::new();
            sv.name = loc.source_file.clone();
            sv.content = match self.metainfo.source_files.get(&sv.name) {
                Some(s) => Some(s.content.clone()),
                None => None,
            };
            self.sv = sv;
        }

        let sv = &mut self.sv;
        sv.highlights = diag.locs.clone();
        self.scroll_into_view();
        let (col, row) = self.sv.cursor;
        self.current_nodes = self.nodes_at(row.into(), col.into());
    }

    pub fn scroll_into_view(&mut self) {
        let selected = &self.diagnostics.selected().unwrap();
        self.sv.highlights = selected.locs.clone();
        let current = selected.current().unwrap();
        self.sv.cursor = ((current.start_col) as u16, (current.start_line - 1) as u16);
    }
}
