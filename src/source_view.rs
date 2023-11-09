use ratatui::{
    prelude::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::cmp;

use crate::parsers::loc_file::Loc;

pub struct SourceView {
    pub name: String,
    pub source: String,
    pub highlights: Vec<Loc>,
}

impl SourceView {
    pub fn render(self, frame: &mut Frame, area: Rect) {
        let mut curr = 0;
        let mut h = self.highlights.get(curr).unwrap();
        let source_lines = self.source.split("\n");
        let mut lines = vec![];

        for (i, line) in source_lines.enumerate() {
            let j = i + 1;
            let line_no = Span::styled(
                format!("{}  ", j),
                Style::default().add_modifier(Modifier::ITALIC),
            );
            let mut content = vec![line_no];

            if j == h.start_line && h.start_line != h.end_line {
                let (a, b) = line.split_at(h.start_col - 1);
                let a = Span::raw(a);
                let b = Span::styled(b, Style::default().bg(ratatui::style::Color::Green));
                content.append(&mut vec![a, b]);
            } else if j == h.start_line {
                let (a, b) = line.split_at(h.start_col - 1);
                let a = Span::raw(a);
                let (b, c) = b.split_at((h.end_col) - (h.start_col - 1));
                let b = Span::styled(b, Style::default().bg(ratatui::style::Color::Green));
                let c = Span::raw(c);
                content.append(&mut vec![a, b, c]);
                curr = cmp::min(curr + 1, self.highlights.len() - 1);
                h = self.highlights.get(curr).unwrap();
            } else if j == h.end_line {
                let (a, b) = line.split_at(h.end_col - 1);
                let a = Span::styled(a, Style::default().bg(ratatui::style::Color::Green));
                let b = Span::raw(b);
                content.append(&mut vec![a, b]);
                curr = cmp::min(curr + 1, self.highlights.len() - 1);
                h = self.highlights.get(curr).unwrap();
            } else {
                content.push(line.into());
            }

            lines.push(Line::from(content));
        }

        frame.render_widget(
            Paragraph::new(lines).block(Block::new().borders(Borders::ALL).title(self.name)),
            area,
        );
    }
}
