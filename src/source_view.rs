use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::parsers::Loc;
pub struct SourceView {
    pub name: String,
    pub content: String,
    pub highlights: Vec<Loc>,
    // Note: This is (y, x) and not (x, y)
    pub scroll: (u16, u16),
    // (x, y)
    pub cursor: (u16, u16),
    line_padding: usize,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl SourceView {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            content: String::new(),
            highlights: Vec::new(),
            scroll: (0, 0),
            cursor: (0, 0),
            line_padding: 0,
        }
    }

    fn get_color(n: &usize) -> Style {
        match n % 5 {
            1 => Style::default().bg(ratatui::style::Color::Green),
            2 => Style::default().bg(ratatui::style::Color::Red),
            3 => Style::default().bg(ratatui::style::Color::Blue),
            4 => Style::default().bg(ratatui::style::Color::Yellow),
            _ => Style::default(),
        }
    }

    pub fn get_widget<'a>(&mut self) -> Paragraph<'a> {
        if self.highlights.is_empty() {
            return Paragraph::new(self.content.clone());
        }
        let source_lines = self.content.lines();
        let n_lines = source_lines.clone().count().to_string().len();
        self.line_padding = n_lines;
        let mut lines = Vec::new();

        for (i, line) in source_lines.enumerate() {
            let line = line.to_owned();
            let j = i + 1;
            let line_no = Span::from(format!("{:>pad$} ", j, pad = self.line_padding));
            let mut content = vec![line_no];
            let highlights_for_line = self
                .highlights
                .iter()
                .filter(|h| h.start_line == j)
                .collect::<Vec<_>>();

            // if self.cursor.1 == i as u16 && self.cursor.0 > line.len() as u16 {
            //     self.cursor.0 = line.len() as u16;
            // }

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

        let p = Paragraph::new(lines);

        p.scroll(self.scroll)
    }

    // TODO: constrain cursor to content but preserve column like vim
    pub fn move_cursor(&mut self, m: Direction) {
        match m {
            Direction::Up => {
                self.cursor.1 = match self.cursor.1 {
                    0 => 0,
                    x => x - 1,
                };
            }
            Direction::Down => {
                self.cursor.1 += 1;
            }
            Direction::Left => {
                self.cursor.0 = match self.cursor.0 {
                    0 => 0,
                    x => x - 1,
                };
            }
            Direction::Right => {
                self.cursor.0 += 1;
            }
        }
    }

    pub fn update_scroll(&mut self, container: &ratatui::prelude::Rect) {
        let lines_in_view = container.height - 2;
        if self.cursor.1 >= self.scroll.0 + lines_in_view {
            self.scroll.0 = self.cursor.1 - lines_in_view + 1;
        } else if self.cursor.1 <= self.scroll.0 {
            self.scroll.0 = self.cursor.1;
        }
    }

    pub fn global_cursor(&self, container: &ratatui::prelude::Rect) -> (u16, u16) {
        let x = std::cmp::min(self.cursor.0 + container.x + 1, container.width - 2);
        let y = std::cmp::min(
            self.cursor.1 + container.y + 1 - self.scroll.0,
            container.y + container.height - 2,
        );

        (x + self.line_padding as u16 + 1, y)
    }
}
