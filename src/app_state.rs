use crate::{
    list::{self, List},
    parsers::{self, MetaInfo, Relation},
    source_view::SourceView,
};

#[derive(PartialEq)]
pub enum AppFocus {
    Relations,
    Tuples,
    Source,
    FilePicker,
    LinePicker,
}

impl AppFocus {
    pub fn next(&self) -> AppFocus {
        match self {
            AppFocus::Tuples => AppFocus::Source,
            AppFocus::Source => AppFocus::Relations,
            AppFocus::Relations => AppFocus::Tuples,
            _ => panic!("invalid call to next"),
        }
    }

    pub fn prev(&self) -> AppFocus {
        match self {
            AppFocus::Tuples => AppFocus::Relations,
            AppFocus::Source => AppFocus::Tuples,
            AppFocus::Relations => AppFocus::Source,
            _ => panic!("invalid call to next"),
        }
    }
}

pub struct AppState {
    pub metainfo: MetaInfo,
    pub tuples: List<parsers::Tuple>,
    pub files: List<String>,
    pub relations: List<parsers::Relation>,
    pub should_quit: bool,
    pub focus: AppFocus,
    pub sv: SourceView,
    pub current_nodes: Vec<usize>,
    pub input_buffer: String,
}

impl AppState {
    pub fn new(metainfo: MetaInfo, files: List<String>) -> Self {
        let mut relation_names = metainfo
            .analyses
            .iter()
            .map(|d| d.name.clone())
            .collect::<Vec<_>>();
        relation_names.dedup();
        let relations = relation_names
            .into_iter()
            .map(|s| Relation::new(s))
            .collect::<Vec<_>>();

        Self {
            metainfo,
            relations: List::new(relations),
            tuples: List::new(vec![]),
            files,
            should_quit: false,
            focus: AppFocus::Relations,
            sv: SourceView::new(),
            current_nodes: vec![],
            input_buffer: String::new(),
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
        let (col, row) = self.sv.get_cursor();
        self.current_nodes = self.nodes_at(row.into(), col.into());
        self.tuples
            .mark(|d| d.nodes.iter().any(|n| self.current_nodes.contains(n)));
    }

    pub fn get_current_tuples(&self) -> Vec<parsers::Tuple> {
        self.metainfo.get_analyses(&self.current_nodes)
    }

    pub fn get_tuples_for_relation(&mut self, relation: &str) {
        let tuples = self.metainfo.get_tuples_for_relation(relation);
        let mut l = vec![];
        for d in &tuples {
            l.push(d.clone());
        }
        let tuples = list::List::new(l);
        self.tuples = tuples;
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
        if self.tuples.selected().is_none() {
            return;
        }
        let tuples = self.tuples.selected().unwrap();
        let loc = tuples.current().unwrap();

        if self.sv.name != loc.source_file {
            let mut sv = SourceView::new();
            sv.name = loc.source_file.clone();
            sv.content = self
                .metainfo
                .source_files
                .get(&sv.name)
                .map(|s| s.content.clone());
            self.sv = sv;
        }

        let sv = &mut self.sv;
        sv.highlights = tuples.locs.clone();
        self.scroll_into_view();
        let (col, row) = self.sv.get_cursor();
        self.current_nodes = self.nodes_at(row.into(), col.into());
    }

    pub fn scroll_into_view(&mut self) {
        let selected = &self.tuples.selected().unwrap();
        self.sv.highlights = selected.locs.clone();
        let current = selected.current().unwrap();
        let target = ((current.start_col) as u16, (current.start_line - 1) as u16);
        self.sv.move_to(target);
    }
}
