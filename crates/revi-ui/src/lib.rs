#![allow(unused)]
#![warn(clippy::all, clippy::pedantic)]
mod key;
mod ui;
pub use crossterm::style::Stylize;
pub use crossterm::{cursor, queue, style};
pub use key::Key;
pub use key::Keys;
pub use ui::screen_size;
pub use ui::Tui;

pub fn clear(stdout: &mut std::io::Stdout) {
    crossterm::execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}

pub trait Display<D: std::fmt::Display> {
    fn render(&mut self, func: impl FnMut(u16, u16, Vec<D>));
    fn cursor(&self, func: impl FnMut(u16, u16, Option<CursorShape>));
}

pub mod widget {
    use super::layout::Rect;
    use std::io::Stdout;
    pub trait Widget: std::fmt::Debug {
        fn x(&self) -> u16;
        fn y(&self) -> u16;
        fn width(&self) -> u16;
        fn height(&self) -> u16;
        fn draw(&self, stdout: &mut Stdout, bounds: Rect);
    }

    #[derive(Debug)]
    pub struct BoxWidget {
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

        pub fn x(&self) -> u16 {
            self.widget.x()
        }
        pub fn y(&self) -> u16 {
            self.widget.y()
        }
        pub fn width(&self) -> u16 {
            self.widget.width()
        }
        pub fn height(&self) -> u16 {
            self.widget.height()
        }
        pub fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
            self.widget.draw(stdout, bounds);
        }
    }
}
pub mod container {
    use super::layout::{Pos, Rect, Size, Stack};
    use super::widget::{BoxWidget, Widget};
    use std::io::Stdout;
    #[derive(Debug, Default)]
    pub struct Container {
        pub bounds: Rect,
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

        pub fn with_bounds(bounds: Rect) -> Self {
            Self {
                bounds,
                stack: Stack::default(),
                children: Vec::new(),
            }
        }

        pub fn stack(mut self, stack: Stack) -> Self {
            self.stack = stack;
            self
        }

        pub fn push<W>(mut self, widget: W) -> Self
        where
            W: Widget + 'static,
        {
            // let width = widget.width().min(self.bounds.width());
            // let height = widget.height().min(self.bounds.height());
            // self.bounds = Rect::new(Size::new(width, height));
            self.children.push(BoxWidget::new(widget));
            self
        }
    }

    impl Widget for Container {
        fn x(&self) -> u16 {
            self.bounds.x()
        }
        fn y(&self) -> u16 {
            self.bounds.y()
        }
        fn width(&self) -> u16 {
            self.bounds.width()
        }
        fn height(&self) -> u16 {
            self.bounds.height()
        }
        fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
            for (widget, wbounds) in self.children.iter().zip(generate_layout(
                bounds,
                self.bounds,
                &self.children,
                self.stack,
            )) {
                widget.draw(stdout, wbounds);
            }
        }
    }

    fn generate_layout(
        root: Rect,
        current: Rect,
        children: &[BoxWidget],
        stack: Stack,
    ) -> Vec<Rect> {
        let x = root.x() + current.x();
        let y = root.y() + current.y();
        // println!("X: {}, root: {}, current: {}", x, root.x(), current.x());
        // println!("Y: {}, root: {}, current: {}", y, root.y(), current.y());

        let count = children.len() as u16;
        // ?
        let width = current.width(); // current.width().min(root.width());
                                     // ?
        let height = current.height(); // current.height().min(root.height());

        let mut layout = vec![Rect::with_position(
            Pos::new(x, y),
            Size::new(width, height),
        )];
        for child in children.iter() {
            if let Some(bounds) = layout.last().cloned() {
                // FIXME: I think its messed up here.
                let width = child.width().min(current.width());
                let height = child.height().min(current.height());
                let (x, y) = match stack {
                    Stack::Vertically => {
                        // println!(
                        //     "child x: {}, height: {} + child y {}",
                        //     child.x(),
                        //     height,
                        //     child.y()
                        // );
                        (child.x(), height + child.y())
                    }
                    Stack::Horizontally => {
                        // println!(
                        //     "width {} + child x: {}, child y {}",
                        //     width,
                        //     child.x(),
                        //     child.y()
                        // );
                        (width + child.x(), child.y())
                    }
                };
                // let (width, height) = match stack {
                //     Stack::Vertically => (bounds.width(), root.height() / count as u16),
                //     Stack::Horizontally => (root.width() / count as u16, bounds.height()),
                // };

                // dbg
                // println!("gen: x: {x}, y: {y}");
                layout.push(Rect::with_position(
                    Pos::new(x, y),
                    Size::new(width, height),
                ));
            }
        }
        layout
    }

    #[test]
    fn check_generate_layout() {
        use super::text::Text;
        let root = Rect::with_position(Pos::new(0, 0), Size::new(20, 20));
        let bounds = Rect::with_position(Pos::new(0, 0), Size::new(0, 0));
        let status_bar = Text::new("Normal Mode, filename goes here").max_height(1);
        let command_bar = Text::new("Command Bar, insert command here").max_height(1);
        let children: Vec<BoxWidget> =
            vec![BoxWidget::new(status_bar), BoxWidget::new(command_bar)];
        let stack = Stack::Vertically;
        assert_eq!(generate_layout(root, bounds, &children, stack), vec![]);
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
    use std::io::{Stdout, Write};
    #[derive(Debug, Default, Clone)]
    pub struct Text {
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

        pub fn max_height(mut self, height: u16) -> Self {
            self.height = height;
            self
        }

        pub fn max_width(mut self, width: u16) -> Self {
            self.width = width;
            self
        }
    }

    impl Widget for Text {
        fn x(&self) -> u16 {
            0
        }
        fn y(&self) -> u16 {
            0
        }
        fn width(&self) -> u16 {
            self.width
        }
        fn height(&self) -> u16 {
            self.height
        }
        fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
            let width = self.width().min(bounds.width()) as usize;
            let height = self.height().min(bounds.height()) as usize;
            // eprintln!(
            //     "w: {} {}, h: {} {}",
            //     width,
            //     height,
            //     bounds.width(),
            //     bounds.height()
            // );
            // eprintln!("x: {}, y: {}", bounds.x(), bounds.y(),);

            for (i, line) in self.content.lines().enumerate().take(height) {
                // eprintln!("      {i} x: {}, y: {}", bounds.x(), bounds.y() + i as u16);
                queue!(
                    stdout,
                    cursor::MoveTo(bounds.x(), bounds.y() + i as u16),
                    style::Print(format_line(line, width)),
                )
                .expect("Failed to queue Text");
                // TODO move this out of for loop after debugging
                stdout.flush().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            // stdout.flush().unwrap();
        }
    }

    /// Shortens or fills line to fit width.
    /// Use this to keep text in side of bounds.
    fn format_line(line: &str, width: usize) -> String {
        // 9608 is the block char for debugging
        let blank = std::char::from_u32(9608).unwrap_or('&');
        line.chars()
            .chain(std::iter::repeat(blank))
            .take(width)
            .collect::<String>()
    }

    impl From<Text> for BoxWidget {
        fn from(text: Text) -> Self {
            BoxWidget::new(text)
        }
    }
}

pub mod layout {
    #[derive(Debug, Clone)]
    pub struct Layout {
        bounds: Rect,
        children: Vec<Rect>,
    }

    impl Layout {
        pub fn new(bounds: Rect) -> Self {
            Self::with_children(bounds, Vec::new())
        }

        pub fn with_children(bounds: Rect, children: Vec<Rect>) -> Self {
            Self { bounds, children }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Stack {
        Vertically,
        Horizontally,
    }

    impl Default for Stack {
        fn default() -> Self {
            Self::Horizontally
        }
    }

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
