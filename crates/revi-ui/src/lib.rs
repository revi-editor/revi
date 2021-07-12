mod key;
mod ui;
pub use key::Key;
pub use ui::screen_size;
pub use ui::Tui;

pub trait Display<B> {
    fn render<F: FnMut(u16, u16, String)>(&self, buffer: &B, func: F);
    fn line_numbers<F: FnMut(u16, u16, String)>(&self, buffer: &B, func: F);
    fn status_bar<F: FnMut(u16, u16, String)>(&self, buffer: &B, func: F);
    fn cursor<F: FnMut(u16, u16, Option<CursorShape>)>(&self, buffer: &B, func: F);
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
