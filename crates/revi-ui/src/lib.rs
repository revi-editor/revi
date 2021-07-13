mod key;
mod ui;
pub use key::Key;
pub use ui::screen_size;
pub use ui::Tui;

pub trait Display {
    fn render(&self, func: impl FnMut(u16, u16, String));
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
        use crossterm::cursor::CursorShape::*;
        match cs {
            CursorShape::UnderScore => UnderScore,
            CursorShape::Line => Line,
            CursorShape::Block => Block,
        }
    }
}
