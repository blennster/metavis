use std::rc::Rc;

use ratatui::text::Text;

use super::{lib::SourceFile, loc_file::Loc};

#[derive(Clone)]
pub struct Diagnostic {
    pub name: String,
    pub source: Rc<SourceFile>,
    pub nodes: Vec<usize>,
    pub locs: Vec<Loc>,
}

impl<'a> From<Diagnostic> for Text<'a> {
    fn from(val: Diagnostic) -> Self {
        Text::from(format!(
            "{}: {}",
            val.name,
            val.nodes
                .iter()
                .map(|n| format!("{}", n)) // TODO: Show locs
                .collect::<Vec<_>>()
                .join(",")
        ))
    }
}
