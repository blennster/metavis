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

    // TODO: Highlight for a line can be seen as a tree or stack
    pub fn get_widget<'a>(self) -> Paragraph<'a> {
        if self.highlights.is_empty() {
            return Paragraph::new(self.source);
        }
        let source_lines = self.source.split("\n");
        let mut lines = Vec::new();

        for (i, line) in source_lines.enumerate() {
            let line = line.to_owned();
            let j = i + 1;
            let line_no = Span::from(
                format!("{:>3} ", j), // TODO: Pad with a number related to the line number count
            );
            let mut content = vec![line_no];
            let mut highlights_for_line = self
                .highlights
                .iter()
                .filter(|h| h.start_line == j)
                .collect::<Vec<_>>();

            if highlights_for_line.is_empty() {
                content.push(Span::raw(line));
                lines.push(Line::from(content));
                continue;
            }

            highlights_for_line.sort_by_key(|a| a.start_col);
            highlights_for_line.reverse();

            let mut split_offset = 0;
            let mut continuation = line;
            while !highlights_for_line.is_empty() && continuation.len() > 0 {
                let h1 = highlights_for_line.pop().unwrap();
                let split_end = highlights_for_line
                    .iter()
                    .map(|h2| h2.start_col)
                    .find(|h2| *h2 < h1.end_col)
                    .unwrap_or(h1.end_col);

                let color = match split_end == h1.end_col {
                    true => Style::default().bg(ratatui::style::Color::Green),
                    false => Style::default().bg(ratatui::style::Color::Red),
                };

                let (a, b) = continuation.split_at((h1.start_col - 1) - split_offset);
                let a = Span::raw(a.to_owned());
                let b = {
                    if h1.end_line == j {
                        let offset = match split_end == h1.end_col {
                            true => 0,
                            false => 1,
                        };
                        let (b, c) = b.split_at((split_end - offset) - (h1.start_col - 1));
                        let b = Span::styled(b.to_owned(), color);
                        split_offset = split_end - 1;
                        continuation = c.to_owned();
                        b
                    } else {
                        let b = Span::styled(b.to_owned(), color);
                        continuation = String::from("");
                        b
                    }
                };
                content.push(a);
                content.push(b);
            }

            if continuation.len() > 0 {
                content.push(Span::styled(continuation, Style::default()));
            }

            // TODO: Render highlights within other highlights
            // if j == h.start_line && h.start_line != h.end_line {
            //     let (a, b) = line.split_at(h.start_col - 1);
            //     let a = Span::raw(a.to_owned());
            //     let b = Span::styled(b.to_owned(), Self::get_color(&curr));
            //     content.append(&mut vec![a, b]);
            // } else if j == h.start_line {
            //     let (a, b) = line.split_at(h.start_col - 1);
            //     let a = Span::raw(a.to_owned());
            //     let (b, c) = b.split_at((h.end_col) - (h.start_col - 1));
            //     let b = Span::styled(b.to_owned(), Self::get_color(&curr));
            //     let c = Span::raw(c.to_owned());
            //     content.append(&mut vec![a, b, c]);
            //     curr = cmp::min(curr + 1, self.highlights.len() - 1);
            //     h = self.highlights.get(curr).unwrap();
            // } else if j == h.end_line {
            //     let (a, b) = line.split_at(h.end_col - 1);
            //     let a = Span::styled(a.to_owned(), Self::get_color(&curr));
            //     let b = Span::raw(b.to_owned());
            //     content.append(&mut vec![a, b]);
            //     curr = cmp::min(curr + 1, self.highlights.len() - 1);
            //     h = self.highlights.get(curr).unwrap();
            // } else {
            //     content.push(line.into());
            // }

            lines.push(Line::from(content));
        }

        Paragraph::new(lines)
    }
}
