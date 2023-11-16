use std::rc::Rc;

use ratatui::text::Text;

use super::loc_file::Loc;

#[derive(Clone)]
pub struct Diagnostic {
    pub name: String,
    pub source_file: String,
    pub source: Rc<str>,
    pub nodes: Vec<usize>,
    pub locs: Vec<Loc>,
}

impl<'a> Into<Text<'a>> for Diagnostic {
    fn into(self) -> Text<'a> {
        Text::from(format!(
            "{}: {}",
            self.name,
            self.nodes
                .iter()
                .map(|n| format!("{}", n)) // TODO: Show locs
                .collect::<Vec<_>>()
                .join(",")
        ))
    }
}