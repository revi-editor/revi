use super::Mode;
use crate::buffer;
use ratatui::{
    layout::{Position, Rect, Size},
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Theme {
    fg: Color,
    bg: Color,
    highlight: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            fg: Color::White,
            bg: Color::Rgb(40, 40, 40),
            highlight: Color::Gray,
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub status_bar: StatusBar,
    pub line_number: LineNumbers,
    pub panes: Vec<Pane>,
    pub current_pane: usize,
}

impl App {
    pub fn get_cursor_position(&self, area: Rect) -> Rect {
        let (_, _, text_layout) = self.setup_layout(area);
        text_layout[self.current_pane]
    }

    pub fn setup_layout(&self, area: Rect) -> (Rc<[Rect]>, Rc<[Rect]>, Rc<[Rect]>) {
        let constraints = self
            .panes
            .iter()
            .map(|_| Constraint::Min(1))
            .collect::<Vec<_>>();
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(area);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Max(3), Constraint::Min(1)].as_ref())
            .split(main_layout[0]);
        let text_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(layout[1]);
        (main_layout, layout, text_layout)
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (main_layout, layout, text_layout) = self.setup_layout(area);

        for (idx, pane) in self.panes.into_iter().enumerate() {
            pane.render(text_layout[idx], buf);
        }
        self.line_number.render(layout[0], buf);
        self.status_bar.render(main_layout[1], buf);
    }
}

#[derive(Debug, Clone)]
pub struct Pane {
    pub buffer: buffer::Buffer,
    pub theme: Theme,
}

impl Widget for Pane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text_on_screen = self
            .buffer
            .on_screen(area.width, area.height)
            .into_iter()
            .map(Line::from)
            .collect::<Vec<Line>>();
        let block = Block::default().borders(Borders::LEFT);
        Paragraph::new(text_on_screen)
            .block(block)
            .style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
            .render(area, buf);
    }
}

#[derive(Debug, Clone, Default)]
pub struct LineNumbers {
    pub theme: Theme,
}

impl Widget for LineNumbers {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = (1..area.height + 1).fold(Vec::new(), |mut acc, num| {
            acc.push(Line {
                spans: vec![format!("{num}").into()],
                style: Style::default().bg(self.theme.bg).fg(self.theme.fg).dim(),
                alignment: Some(Alignment::Right),
            });
            acc
        });

        Paragraph::new(text)
            .style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
            .render(area, buf);
    }
}

#[derive(Debug, Clone)]
pub struct StatusBar {
    pub mode: Mode,
    pub cursor: Position,
    pub filename: String,
    pub file_saved: bool,
    pub theme: Theme,
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let file_saved = if self.file_saved { "✔ " } else { "✘ " };
        let file_saved_style = if self.file_saved {
            // TODO: make colors configurable
            Style::default().green().on_dark_gray().bold()
        } else {
            Style::default().red().on_dark_gray().bold()
        };
        Line::from(vec![
            Span::styled(
                format!("{:?} ", self.mode),
                Style::default().bg(self.theme.bg).fg(self.theme.fg).bold(),
            ),
            Span::styled(
                format!("{} ", self.filename),
                Style::default().bg(self.theme.bg).fg(self.theme.fg).bold(),
            ),
            Span::styled(file_saved, file_saved_style),
        ])
        .render(area, buf);
        Line {
            spans: vec![Span::raw(format!("{}/{}", self.cursor.x, self.cursor.y))],
            style: Style::default()
                .bg(self.theme.highlight)
                .fg(self.theme.fg)
                .bold(),
            alignment: Some(Alignment::Right),
        }
        .render(area, buf);
    }
}
