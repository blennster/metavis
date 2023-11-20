use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::parsers::Loc;
pub struct SourceView {
    pub content: String,
    pub highlights: Vec<Loc>,
}

impl SourceView {
    fn get_color(n: &usize) -> Style {
        match n {
            0 => Style::default(),
            1 => Style::default().bg(ratatui::style::Color::Green),
            2 => Style::default().bg(ratatui::style::Color::Red),
            3 => Style::default().bg(ratatui::style::Color::Blue),
            _ => Style::default().bg(ratatui::style::Color::Yellow),
        }
    }

    pub fn get_widget<'a>(self) -> Paragraph<'a> {
        if self.highlights.is_empty() {
            return Paragraph::new(self.content);
        }
        let source_lines = self.content.split("\n");
        let mut lines = Vec::new();

        for (i, line) in source_lines.enumerate() {
            let line = line.to_owned();
            let j = i + 1;
            let line_no = Span::from(
                format!("{:>3} ", j), // TODO: Pad with a number related to the line number count
            );
            let mut content = vec![line_no];
            let highlights_for_line = self
                .highlights
                .iter()
                .filter(|h| h.start_line == j)
                .collect::<Vec<_>>();

            if highlights_for_line.is_empty() {
                content.push(Span::raw(line));
                lines.push(Line::from(content));
                continue;
            }

            let mut acc = String::new();

            // TODO: Map highlight color to a node id (maybe)
            let mut old_level = 0;
            let mut level = 0;
            for (k, c) in line.char_indices() {
                let k = k + 1;
                level = highlights_for_line
                    .iter()
                    .filter(|h| h.start_col <= k && k <= h.end_col)
                    .collect::<Vec<_>>()
                    .len();

                if level != old_level {
                    content.push(Span::styled(acc, Self::get_color(&old_level)));
                    acc = String::new();
                    old_level = level;
                }

                acc.push(c);
            }
            content.push(Span::styled(acc, Self::get_color(&level)));

            lines.push(Line::from(content));
        }

        Paragraph::new(lines)
    }
}
