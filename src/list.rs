use std::cmp;

use ratatui::widgets::ListItem;

pub struct List<T>
// where
//     for<'a> &'a T: Into<ListItem<'a>>,
{
    pub items: Vec<T>,
    pub marked: Vec<usize>,
    selected: Option<usize>,
    pub state: ratatui::widgets::ListState,
}

impl<T> List<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            marked: vec![],
            selected: None,
            state: ratatui::widgets::ListState::default().with_selected(Some(0)),
        }
    }
    pub fn down(&mut self) {
        let len = self.items.len();
        self.state.select(Some(cmp::min(
            len - 1,
            self.state.selected().unwrap_or(0) + 1,
        )));
    }

    pub fn up(&mut self) {
        self.state.select(match self.state.selected() {
            Some(0) => Some(0),
            Some(x) => Some(x - 1),
            None => Some(0),
        });
    }

    pub fn confirm(&mut self) {
        self.selected = self.state.selected();
    }

    // Mark one or many items based on some filter
    pub fn mark(&mut self, func: impl Fn(&T) -> bool) {
        self.marked = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if func(t) { Some(i) } else { None })
            .collect();
    }

    pub fn selected(&mut self) -> Option<&mut T> {
        match self.selected {
            Some(x) => self.items.get_mut(x),
            None => None,
        }
    }
}

impl<T> List<T>
where
    for<'a> T: Into<ratatui::text::Text<'a>>,
    T: Clone,
{
    pub fn widget<'a>(&self) -> ratatui::widgets::List<'a> {
        let mut items = self
            .items
            .iter()
            .map(|i| i.clone().into())
            .collect::<Vec<ratatui::text::Text>>();

        for i in self.marked.iter() {
            items[*i]
                .patch_style(ratatui::style::Style::default().bg(ratatui::style::Color::Green));
        }

        let list_items = items.into_iter().map(ListItem::new).collect::<Vec<_>>();
        ratatui::widgets::List::new(list_items).highlight_symbol(">")
    }
}
