use std::rc::Rc;

use ratatui::widgets::ListItem;

use super::loc_file::Loc;

pub struct Diagnostic {
    pub name: String,
    pub source_file: String,
    pub source: Rc<str>,
    pub nodes: Vec<usize>,
    pub locs: Vec<Loc>,
}

impl<'a> Into<ListItem<'a>> for &Diagnostic {
    fn into(self) -> ListItem<'a> {
        ListItem::new(format!(
            "{}: {}",
            self.name,
            self.nodes
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ))
    }
}
