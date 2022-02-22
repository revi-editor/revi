#![allow(unused)]
#![warn(clippy::all, clippy::pedantic)]
mod key;
mod ui;
pub use crossterm::style::Stylize;
pub use key::Key;
pub use key::Keys;
pub use ui::screen_size;
pub use ui::Tui;

pub trait Display<D: std::fmt::Display> {
    fn render(&mut self, func: impl FnMut(u16, u16, Vec<D>));
    fn cursor(&self, func: impl FnMut(u16, u16, Option<CursorShape>));
}

pub mod widget {
    use super::layout::Rect;
    use std::io::Stdout;
    pub trait Widget {
        fn width(&self) -> u16;
        fn height(&self) -> u16;
        fn draw(&self, stdout: &mut Stdout, bounds: Rect);
    }

    pub(crate) struct BoxWidget {
        widget: Box<dyn Widget>,
    }

    impl BoxWidget {
        pub fn new<W>(widget: W) -> Self
        where
            W: Widget + 'static,
        {
            Self {
                widget: Box::new(widget),
            }
        }

        pub fn width(&self) -> u16 {
            self.widget.width()
        }
        pub fn height(&self) -> u16 {
            self.widget.height()
        }
        pub fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
            self.widget.draw(stdout, bounds)
        }
    }
}
pub mod container {
    use super::layout::{Pos, Rect, Size, Stack};
    use super::widget::{BoxWidget, Widget};
    use std::io::Stdout;
    struct Container {
        bounds: Rect,
        stack: Stack,
        children: Vec<BoxWidget>,
    }

    impl Container {
        pub fn new(bounds: Rect, stack: Stack) -> Self {
            Self {
                bounds,
                stack,
                children: Vec::new(),
            }
        }
    }

    impl Widget for Container {
        fn width(&self) -> u16 {
            self.bounds.width()
        }
        fn height(&self) -> u16 {
            self.bounds.height()
        }
        fn draw(&self, stdout: &mut Stdout, mut bounds: Rect) {
            for widget in self.children.iter() {
                widget.draw(stdout, bounds.clone());
                bounds = match self.stack {
                    Stack::Vertically => Rect::with_position(
                        Pos::new(bounds.x(), bounds.y() + widget.height()),
                        Size::new(bounds.width(), bounds.height()),
                    ),
                    Stack::Horizontally => Rect::with_position(
                        Pos::new(bounds.x() + widget.width(), bounds.y()),
                        Size::new(bounds.width(), bounds.height()),
                    ),
                };
            }
        }
    }
    impl From<Container> for BoxWidget {
        fn from(container: Container) -> Self {
            BoxWidget::new(container)
        }
    }
}
pub mod text {
    use super::layout::{Pos, Rect, Size, Stack};
    use super::widget::{BoxWidget, Widget};
    use crossterm::{cursor, queue, style};
    use std::io::Stdout;
    struct Text {
        content: String,
        width: u16,
        height: u16,
    }

    impl Text {
        pub fn new(content: &str) -> Self {
            Self {
                content: content.into(),
                width: content.lines().map(|x| x.len()).max().unwrap_or(0) as u16,
                height: content.lines().count() as u16,
            }
        }
    }

    impl Widget for Text {
        fn width(&self) -> u16 {
            self.width
        }
        fn height(&self) -> u16 {
            self.height
        }
        fn draw(&self, stdout: &mut Stdout, mut bounds: Rect) {
            for (i, line) in self.content.lines().enumerate() {
                queue!(
                    stdout,
                    cursor::MoveTo(bounds.x(), bounds.y() + i as u16),
                    style::Print(line)
                )
                .expect("Failed to queue Text");
            }
        }
    }

    impl From<Text> for BoxWidget {
        fn from(text: Text) -> Self {
            BoxWidget::new(text)
        }
    }
}

pub mod layout {
    #[derive(Debug, Clone, Copy)]
    pub enum Stack {
        Vertically,
        Horizontally,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Rect {
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    }

    impl Rect {
        pub fn new(size: Size) -> Self {
            Self::with_position(Pos::new(0, 0), size)
        }
        pub fn with_position(pos: Pos, size: Size) -> Self {
            Self {
                x: pos.x,
                y: pos.y,
                width: size.width,
                height: size.height,
            }
        }
        pub fn x(&self) -> u16 {
            self.x
        }
        pub fn y(&self) -> u16 {
            self.y
        }
        pub fn width(&self) -> u16 {
            self.width
        }
        pub fn height(&self) -> u16 {
            self.height
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Pos {
        x: u16,
        y: u16,
    }

    impl Pos {
        pub fn new(x: u16, y: u16) -> Self {
            Self { x, y }
        }
    }

    pub struct Size {
        width: u16,
        height: u16,
    }

    impl Size {
        pub fn new(width: u16, height: u16) -> Self {
            Self { width, height }
        }
    }
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
