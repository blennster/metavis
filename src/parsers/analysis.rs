use ratatui::text::Text;

use super::loc_file::Loc;

#[derive(Clone)]
pub struct Tuple {
    pub name: String,
    pub nodes: Vec<usize>,
    pub locs: Vec<Loc>,
    current_loc: Option<usize>, // TODO: make private / find another way
}

impl Tuple {
    pub fn new(name: String, nodes: Vec<usize>, locs: Vec<Loc>) -> Self {
        Self {
            name,
            nodes,
            locs,
            current_loc: None,
        }
    }

    pub fn next(&mut self) {
        let c = self.current_loc.unwrap_or(0);
        self.current_loc = Some(std::cmp::min(c + 1, self.nodes.len() - 1));
    }

    pub fn prev(&mut self) {
        self.current_loc = match self.current_loc {
            Some(0) => Some(0),
            Some(x) => Some(x - 1),
            _ => None,
        };
    }

    pub fn unset(&mut self) {
        self.current_loc = None;
    }

    pub fn set(&mut self) {
        self.current_loc = Some(0);
    }

    pub fn current(&self) -> Option<&Loc> {
        self.locs.get(self.current_loc.unwrap_or(0))
    }
}

impl<'a> From<Tuple> for Text<'a> {
    fn from(val: Tuple) -> Self {
        let mut nodes_text = vec![];
        let mut source_files = val
            .locs
            .iter()
            .map(|l| l.source_file.clone())
            .collect::<Vec<_>>();
        source_files.dedup();

        for (i, n) in val.nodes.iter().enumerate() {
            if val.current_loc.is_some() && i == val.current_loc.unwrap() {
                nodes_text.push(format!("*{}*", n));
            } else {
                nodes_text.push(format!("{}", n));
            }
        }

        Text::from(format!(
            "({}) @ {}",
            nodes_text.join(","),
            source_files.join(" & ")
        ))
    }
}

#[derive(Clone)]
pub struct Relation {
    pub name: String,
    current: bool,
}

impl Relation {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current: false,
        }
    }

    pub fn mark(&mut self) {
        self.current = true;
    }

    pub fn unmark(&mut self) {
        self.current = false;
    }
}

impl<'a> From<Relation> for Text<'a> {
    fn from(value: Relation) -> Self {
        if value.current {
            Text::from(format!("*{}*", value.name))
        } else {
            Text::from(value.name)
        }
    }
}
