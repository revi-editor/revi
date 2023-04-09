pub fn clear(stdout: &mut std::io::Stdout) {
    crossterm::execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}

pub fn size() -> (u16, u16) {
    crossterm::terminal::size().unwrap_or((0, 0))
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
        fn debug_name(&self) -> String {
            "DEFAULT".to_string()
        }
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
        pub fn _debug_name(&self) -> String {
            self.widget.debug_name()
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
        comment: Option<String>,
    }

    impl Container {
        pub fn new(bounds: Rect, stack: Stack) -> Self {
            Self {
                bounds,
                stack,
                children: Vec::new(),
                comment: None,
            }
        }

        pub fn _with_bounds(bounds: Rect) -> Self {
            Self {
                bounds,
                stack: Stack::default(),
                children: Vec::new(),
                comment: None,
            }
        }

        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.comment = Some(comment.into());
            self
        }

        pub fn stack(mut self, stack: Stack) -> Self {
            self.stack = stack;
            self
        }

        pub fn push<W>(mut self, widget: W) -> Self
        where
            W: Widget + 'static,
        {
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
        fn debug_name(&self) -> String {
            self.comment.clone().unwrap_or_default()
        }
    }

    fn generate_layout(
        root: Rect,
        current: Rect,
        children: &[BoxWidget],
        stack: Stack,
    ) -> Vec<Rect> {
        children.iter().fold(vec![], |mut acc, child| {
            let last = acc.last().cloned().unwrap_or_default();
            let x = match stack {
                Stack::Vertically => current.x() + child.x() + root.x(),
                Stack::Horizontally => current.x() + child.x() + last.width() + last.x() + root.x(),
            };
            let y = match stack {
                Stack::Vertically => current.y() + child.y() + last.height() + last.y() + root.y(),
                Stack::Horizontally => current.y() + child.y() + root.y(),
            };
            let width = match stack {
                Stack::Vertically => child.width().min(current.width()),
                Stack::Horizontally => child.width().min(current.width() - last.width()),
            };
            let height = match stack {
                Stack::Vertically => child.height().min(current.height() - last.height()),
                Stack::Horizontally => child.height().min(current.height()),
            };
            let size = Size::new(width, height);
            let pos = Pos::new(x, y);
            let rect = Rect::with_position(pos, size);
            acc.push(rect);
            acc
        })
    }

    impl From<Container> for BoxWidget {
        fn from(container: Container) -> Self {
            BoxWidget::new(container)
        }
    }
}
pub mod text {
    use super::layout::Rect;
    use super::widget::{BoxWidget, Widget};
    use crossterm::{cursor, queue, style};
    use std::io::{Stdout, Write};
    #[derive(Debug, Default, Clone)]
    pub struct Text {
        content: String,
        width: u16,
        height: u16,
        comment: Option<String>,
    }

    impl Text {
        pub fn new(content: &str) -> Self {
            Self {
                content: content.into(),
                width: content.lines().map(|x| x.len()).max().unwrap_or(0) as u16,
                height: content.lines().count() as u16,
                comment: None,
            }
        }

        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.comment = Some(comment.into());
            self
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
            for (i, line) in self
                .content
                .lines()
                .enumerate()
                .take(bounds.height() as usize)
            {
                queue!(
                    stdout,
                    cursor::MoveTo(bounds.x(), bounds.y() + i as u16),
                    style::Print(format_line(line, bounds.width() as usize)),
                )
                .expect("Failed to queue Text");
                // TODO: move this out of for loop after debugging
                stdout.flush().unwrap();
                // std::thread::sleep(std::time::Duration::from_millis(300));
            }
            // stdout.flush().unwrap();
        }
        fn debug_name(&self) -> String {
            self.comment.clone().unwrap_or_default()
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
    // #[derive(Debug, Clone)]
    // pub struct Layout {
    //     bounds: Rect,
    //     children: Vec<Rect>,
    // }
    //
    // impl Layout {
    //     pub fn new(bounds: Rect) -> Self {
    //         Self::with_children(bounds, Vec::new())
    //     }
    //
    //     pub fn with_children(bounds: Rect, children: Vec<Rect>) -> Self {
    //         Self { bounds, children }
    //     }
    // }

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
