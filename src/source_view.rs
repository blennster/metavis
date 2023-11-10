use ratatui::{
    prelude::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};
use std::cmp;

use crate::parsers::Loc;

pub struct SourceView {
    pub source: String,
    pub highlights: Vec<Loc>,
}

impl SourceView {
    fn get_color(n: &usize) -> Style {
        match n {
            0 => Style::default().bg(ratatui::style::Color::Green),
            1 => Style::default().bg(ratatui::style::Color::Red),
            2 => Style::default().bg(ratatui::style::Color::Blue),
            _ => Style::default(),
        }
    }

    pub fn get_widget<'a>(self) -> Paragraph<'a> {
        let mut curr = 0;
        let h = self.highlights.get(curr);
        if h.is_none() {
            return Paragraph::new(self.source);
        }
        let mut h = h.unwrap();
        let source_lines = self.source.split("\n");
        let mut lines = vec![];

        for (i, line) in source_lines.enumerate() {
            let line = line.to_owned();
            let j = i + 1;
            let line_no = Span::styled(
                format!("{:>2}  ", j), // TODO: Pad with a number related to the line number count
                Style::default().add_modifier(Modifier::ITALIC),
            );
            let mut content = vec![line_no];

            // TODO: Render highlights within other highlights
            if j == h.start_line && h.start_line != h.end_line {
                let (a, b) = line.split_at(h.start_col - 1);
                let a = Span::raw(a.to_owned());
                let b = Span::styled(b.to_owned(), Self::get_color(&curr));
                content.append(&mut vec![a, b]);
            } else if j == h.start_line {
                let (a, b) = line.split_at(h.start_col - 1);
                let a = Span::raw(a.to_owned());
                let (b, c) = b.split_at((h.end_col) - (h.start_col - 1));
                let b = Span::styled(b.to_owned(), Self::get_color(&curr));
                let c = Span::raw(c.to_owned());
                content.append(&mut vec![a, b, c]);
                curr = cmp::min(curr + 1, self.highlights.len() - 1);
                h = self.highlights.get(curr).unwrap();
            } else if j == h.end_line {
                let (a, b) = line.split_at(h.end_col - 1);
                let a = Span::styled(a.to_owned(), Self::get_color(&curr));
                let b = Span::raw(b.to_owned());
                content.append(&mut vec![a, b]);
                curr = cmp::min(curr + 1, self.highlights.len() - 1);
                h = self.highlights.get(curr).unwrap();
            } else {
                content.push(line.into());
            }

            lines.push(Line::from(content));
        }

        Paragraph::new(lines)
    }
}
