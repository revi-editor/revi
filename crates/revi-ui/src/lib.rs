#![warn(clippy::all, clippy::pedantic)]
mod key;
mod ui;
pub use crossterm::style::Stylize;
pub use key::Key;
pub use ui::screen_size;
pub use ui::Tui;

pub trait Display {
    fn render(&mut self, func: impl FnMut(u16, u16, Vec<String>));
    fn cursor(&self, func: impl FnMut(u16, u16, Option<CursorShape>));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    UnderScore,
    Line,
    Block,
}

impl From<CursorShape> for crossterm::cursor::CursorShape {
    fn from(cs: CursorShape) -> Self {
        use crossterm::cursor::CursorShape::{Block, Line, UnderScore};
        match cs {
            CursorShape::UnderScore => UnderScore,
            CursorShape::Line => Line,
            CursorShape::Block => Block,
        }
    }
}
