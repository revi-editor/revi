#[must_use]
pub fn size() -> (u16, u16) {
    crossterm::terminal::size().unwrap_or((0, 0))
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

        #[must_use]
        pub fn x(&self) -> u16 {
            self.widget.x()
        }

        #[must_use]
        pub fn y(&self) -> u16 {
            self.widget.y()
        }

        #[must_use]
        pub fn width(&self) -> u16 {
            self.widget.width()
        }

        #[must_use]
        pub fn height(&self) -> u16 {
            self.widget.height()
        }

        pub fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
            self.widget.draw(stdout, bounds);
        }

        #[must_use]
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

        pub fn with_child<W>(mut self, widget: W) -> Self
        where
            W: Widget + 'static,
        {
            self.children.push(BoxWidget::new(widget));
            self
        }

        pub fn with_child_box(mut self, boxed_widget: BoxWidget) -> Self {
            self.children.push(boxed_widget);
            self
        }

        pub fn push<W>(&mut self, widget: W)
        where
            W: Widget + 'static,
        {
            self.children.push(BoxWidget::new(widget));
        }

        pub fn push_box(&mut self, boxed_widget: BoxWidget) {
            self.children.push(boxed_widget);
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
                Stack::Vertically => child.width().min(current.width()).min(root.width()),
                Stack::Horizontally => child
                    .width()
                    .min(current.width() - last.width())
                    .min(root.width()),
            };
            let height = match stack {
                Stack::Vertically => child
                    .height()
                    .min(current.height() - last.height())
                    .min(root.height()),
                Stack::Horizontally => child.height().min(current.height()).min(root.height()),
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
    use crossterm::style::{Attribute, Color, ContentStyle, ResetColor, SetAttribute, SetStyle};
    use crossterm::{cursor, queue, style};
    use std::io::Stdout;
    #[derive(Debug, Default, Clone)]
    pub struct Text {
        content: String,
        style: ContentStyle,
        width: u16,
        height: u16,
        comment: Option<String>,
    }

    impl Text {
        pub fn new(content: &str) -> Self {
            Self {
                content: content.into(),
                style: ContentStyle::new(),
                width: content.lines().map(|x| x.len()).max().unwrap_or_default() as u16,
                height: content.lines().count() as u16,
                comment: None,
            }
        }

        pub fn with_fg(mut self, fg: Color) -> Self {
            self.style.foreground_color = Some(fg);
            self
        }

        pub fn with_bg(mut self, bg: Color) -> Self {
            self.style.background_color = Some(bg);
            self
        }

        pub fn with_atter(mut self, atter: impl Into<style::Attributes>) -> Self {
            self.style.attributes = atter.into();
            self
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
            queue!(stdout, SetStyle(self.style)).expect("failed to set style");
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
            }
            queue!(stdout, ResetColor, SetAttribute(Attribute::Reset))
                .expect("failed to queue reset color and  attribute");
        }
        fn debug_name(&self) -> String {
            self.comment.clone().unwrap_or_default()
        }
    }

    /// Shortens or fills line to fit width.
    /// Use this to keep text in side of bounds.
    fn format_line(line: &str, width: usize) -> String {
        // 9608 is the block char for debugging
        let blank = ' '; // std::char::from_u32(9608).unwrap_or('&');
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

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Rect {
        pub x: u16,
        pub y: u16,
        pub width: u16,
        pub height: u16,
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

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Pos {
        pub x: u16,
        pub y: u16,
    }

    impl Pos {
        pub fn new(x: u16, y: u16) -> Self {
            Self { x, y }
        }
    }

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Size {
        pub width: u16,
        pub height: u16,
    }

    impl Size {
        pub fn new(width: u16, height: u16) -> Self {
            Self { width, height }
        }
    }
}

pub mod application {
    use super::{layout::Pos, widget::BoxWidget};
    use crate::key::Keys;
    use crossterm::{cursor::SetCursorStyle, Result};
    pub trait App: Sized {
        type Settings;
        fn new(_: Self::Settings) -> Self;
        fn update(&mut self, keys: Keys);
        fn view(&self) -> BoxWidget;
        fn cursor(&self) -> (Option<Pos>, Option<SetCursorStyle>) {
            (None, None)
        }
        fn quit(&self) -> bool {
            true
        }
        fn run(&mut self) -> Result<()> {
            crate::tui::runtime::run(self)
        }
    }
}

mod runtime {
    use super::{
        application::App,
        layout::{Pos, Rect, Size},
    };
    use crate::key;
    use crossterm::{
        cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
        event,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        QueueableCommand, Result,
    };
    use std::io::Stdout;
    use std::{io::Write, time::Duration};

    fn render_app<A>(w: &mut Stdout, app: &mut A) -> Result<()>
    where
        A: App,
    {
        w.queue(Hide)?;
        let (cursor_pos, cursor_style) = app.cursor();
        if let Some(Pos { x, y }) = cursor_pos {
            w.queue(MoveTo(x, y))?;
        }
        if let Some(cs) = cursor_style {
            w.queue(cs)?;
        }
        let widgets = app.view();
        let width = widgets.width();
        let height = widgets.height();
        let app_size = Size { width, height };
        let app_pos = Pos { x: 0, y: 0 };
        w.queue(SavePosition)?;
        widgets.draw(w, Rect::with_position(app_pos, app_size));
        w.queue(RestorePosition)?;
        if cursor_style.is_some() {
            w.queue(Show)?;
        }
        w.flush()?;
        Ok(())
    }

    pub fn run<A>(app: &mut A) -> Result<()>
    where
        A: App,
    {
        let mut writer = std::io::stdout();
        writer.queue(EnterAlternateScreen)?;
        writer.queue(SavePosition)?;
        writer.queue(Hide)?;
        enable_raw_mode()?;
        writer.flush()?;

        render_app(&mut writer, app)?;
        while app.quit() {
            // HACK: this is temporary untill other events need to be handled.
            let keys = if event::poll(Duration::from_millis(50)).unwrap_or(false) {
                let event = event::read().expect("failed to read from stdin");
                match event {
                    event::Event::Key(key_event) => key::Keys::from(key_event),
                    _ => key::Keys::Key(key::Key::Null),
                }
            } else {
                key::Keys::Key(key::Key::Null)
            };

            app.update(keys);
            // Update cursor pos on screen after update
            if !matches!(keys, key::Keys::Key(key::Key::Null)) {
                render_app(&mut writer, app)?;
            }
        }

        disable_raw_mode()?;
        writer.queue(LeaveAlternateScreen)?;
        writer.queue(RestorePosition)?;
        writer.queue(Show)?;
        writer.flush()?;
        Ok(())
    }
}
