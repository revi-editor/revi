#![allow(unused)]
mod widget {
    use std::io::Stdout;
    pub trait Widget<Message> {
        fn width(&self) -> u16;
        fn height(&self) -> u16;
        fn draw(&self, stdout: &mut Stdout);
    }
}
mod element {
    use super::widget::Widget;
    use std::io::Stdout;
    pub struct Element<'a, Message> {
        pub widget: Box<dyn Widget<Message> + 'a>,
    }

    impl<'a, Message> Element<'a, Message> {
        pub fn new(widget: impl Widget<Message> + 'a) -> Self {
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
        pub fn draw(&self, stdout: &mut Stdout) {
            self.widget.draw(stdout);
        }
    }
}

mod text {
    use crate::position::Position;

    use super::cursor::Cursor;
    use super::element::Element;
    use super::widget::Widget;
    use crossterm::{cursor, event, execute, queue, style, terminal};
    use std::io::Stdout;
    pub struct Text {
        content: String,
        cursor: Option<Cursor>,
        loc: Position,
    }

    impl Text {
        pub fn new(content: &str) -> Self {
            Self {
                content: content.into(),
                cursor: None,
                loc: Position::default(),
            }
        }
        pub fn with_cursor(mut self, cursor: Cursor) -> Self {
            self.cursor = Some(cursor);
            self
        }
    }

    impl<Message> Widget<Message> for Text {
        fn width(&self) -> u16 {
            self.content.len() as u16
        }
        fn height(&self) -> u16 {
            self.content.lines().fold(0u16, |mut acc, _| {
                acc += 1;
                acc
            })
        }
        fn draw(&self, stdout: &mut Stdout) {
            queue!(
                stdout,
                cursor::Hide,
                cursor::MoveTo(self.loc.row, self.loc.col),
                style::Print(&self.content)
            )
            .expect("Failed to queue view.");
            if let Some(c) = self.cursor {
                queue!(stdout, cursor::MoveTo(c.row, c.col), cursor::Show,)
                    .expect("Failed to queue view.");
            }
        }
    }

    impl<'a, Message> From<Text> for Element<'a, Message> {
        fn from(text: Text) -> Self {
            Element::new(text)
        }
    }
}

mod runtime {
    use super::app::Application;
    use crate::element::Element;
    use crossterm::{cursor, event, execute, queue, style, terminal};
    use std::io::{stdout, Write};
    pub struct Instance<A>(A)
    where
        A: Application;

    impl<A: Application> Drop for Instance<A> {
        fn drop(&mut self) {
            execute!(
                stdout(),
                terminal::LeaveAlternateScreen,
                cursor::RestorePosition,
                cursor::Show,
            )
            .expect("Failed to Exit Alt Screen");
            terminal::disable_raw_mode().expect("Failed to Raw Mode");
        }
    }
    impl<A> runtime::terminal::Program for Instance<A>
    where
        A: Application,
    {
        type Msg = A::Msg;
        fn update(&mut self, msg: &Self::Msg) {
            self.0.update(&msg)
        }
        fn view(&mut self) -> Element<Self::Msg> {
            self.0.view()
        }
        fn subscribe(&mut self, event: event::Event) -> Vec<Self::Msg> {
            self.0.subscribe(event)
        }
    }

    impl<A> runtime::Application for Instance<A>
    where
        A: Application,
    {
        fn new() -> Self {
            Self(A::new())
        }

        fn should_exit(&self) -> bool {
            self.0.should_exit()
        }
    }

    pub fn run<A>()
    where
        A: runtime::Application,
    {
        let mut app = A::new();
        let mut is_running = app.should_exit();
        let mut writer = stdout();
        execute!(
            &mut writer,
            cursor::SavePosition,
            terminal::EnterAlternateScreen,
        )
        .expect("Failed to enter into alternate Screen.");

        terminal::enable_raw_mode().expect("Failed to enter into raw mode.");

        let (_width, _height) = terminal::size().expect("Failed to get terminal size.");
        app.view().draw(&mut writer);
        writer.flush().expect("Failed to flush view");
        while app.should_exit() {
            if event::poll(std::time::Duration::from_millis(50)).expect("Failed to Poll") {
                let events = event::read().expect("Failed to Read Event");
                for msg in app.subscribe(events).iter() {
                    app.update(msg);
                }
                app.view().draw(&mut writer);
                writer.flush().expect("Failed to flush view");
            }
        }
    }

    pub(crate) mod runtime {
        use super::Element;
        use terminal::Program;
        pub trait Application: Program {
            fn new() -> Self;
            fn should_exit(&self) -> bool;
        }

        pub mod terminal {
            use super::Element;
            pub trait Program {
                type Msg;
                fn update(&mut self, msg: &Self::Msg);
                fn view(&mut self) -> Element<Self::Msg>;
                fn subscribe(&mut self, event: crossterm::event::Event) -> Vec<Self::Msg>;
            }
        }
    }
}

mod app {
    use super::element::Element;
    use super::runtime;
    use super::runtime::Instance;
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    pub trait Application {
        type Msg;
        fn new() -> Self;
        fn update(&mut self, msg: &Self::Msg);
        fn view(&self) -> Element<Self::Msg>;
        fn should_exit(&self) -> bool {
            true
        }
        fn subscribe(&mut self, _event: Event) -> Vec<Self::Msg> {
            Vec::new()
        }
        fn run()
        where
            Self: 'static,
            Self: Sized,
        {
            runtime::run::<Instance<Self>>();
        }
    }
}

mod position {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Position {
        pub row: u16,
        pub col: u16,
    }
}

mod cursor {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Cursor {
        pub row: u16,
        pub col: u16,
    }
}
// ----------------------------------------------
// main file
use element::Element;
enum MsgKind {
    Normal(NormalMessage),
}

enum NormalMessage {
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
}

enum Mode {
    Normal,
    Insert,
    Command,
}

struct Revi {
    buffers: Vec<String>,
    mode: Mode,
    cursor: cursor::Cursor,
    is_running: bool,
}

impl app::Application for Revi {
    type Msg = MsgKind;
    fn new() -> Self {
        Self {
            buffers: Vec::new(),
            mode: Mode::Normal,
            cursor: cursor::Cursor::default(),
            is_running: true,
        }
    }

    fn update(&mut self, mode: &Self::Msg) {
        match mode {
            MsgKind::Normal(normal) => match normal {
                NormalMessage::CursorUp => self.cursor.col = self.cursor.col.saturating_sub(1),
                NormalMessage::CursorDown => self.cursor.col += 1,
                NormalMessage::CursorLeft => self.cursor.row = self.cursor.row.saturating_sub(1),
                NormalMessage::CursorRight => self.cursor.row += 1,
            },
        }
    }

    fn should_exit(&self) -> bool {
        self.is_running
    }

    fn view(&self) -> Element<Self::Msg> {
        text::Text::new("Hello World")
            .with_cursor(self.cursor)
            .into()
    }

    fn subscribe(&mut self, event: app::Event) -> Vec<Self::Msg> {
        let mut messages = Vec::new();
        match event {
            app::Event::Key(key_event) => match &self.mode {
                Mode::Normal => match key_event {
                    app::KeyEvent {
                        code: app::KeyCode::Esc,
                        modifiers: app::KeyModifiers::NONE,
                    } => {
                        self.is_running = false;
                    }
                    app::KeyEvent {
                        code: app::KeyCode::Char('j'),
                        modifiers: app::KeyModifiers::NONE,
                    } => {
                        messages.push(MsgKind::Normal(NormalMessage::CursorDown));
                    }
                    app::KeyEvent {
                        code: app::KeyCode::Char('k'),
                        modifiers: app::KeyModifiers::NONE,
                    } => {
                        messages.push(MsgKind::Normal(NormalMessage::CursorUp));
                    }
                    app::KeyEvent {
                        code: app::KeyCode::Char('h'),
                        modifiers: app::KeyModifiers::NONE,
                    } => {
                        messages.push(MsgKind::Normal(NormalMessage::CursorLeft));
                    }
                    app::KeyEvent {
                        code: app::KeyCode::Char('l'),
                        modifiers: app::KeyModifiers::NONE,
                    } => {
                        messages.push(MsgKind::Normal(NormalMessage::CursorRight));
                    }
                    _ => {}
                },
                Mode::Insert => {}
                Mode::Command => {}
            },
            app::Event::Mouse(_) => {}
            app::Event::Resize(_, _) => {}
        }
        messages
    }
}
use app::Application;
fn main() {
    Revi::run();
}
