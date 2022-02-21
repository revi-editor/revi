#![allow(unused)]
mod runtime {
    use super::ui::Application;
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
        fn update(&mut self, msg: Self::Msg) {
            self.0.update(msg)
        }
        fn view(&mut self) -> String {
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
        let views = app.view();
        while app.should_exit() {
            if event::poll(std::time::Duration::from_millis(50)).expect("Failed to Poll") {
                let events = event::read().expect("Failed to Read Event");
                app.subscribe(events);
                queue!(&mut writer, cursor::MoveTo(0, 0), style::Print(&views))
                    .expect("Failed to queue view.");
            }
            writer.flush().expect("Failed to flush view");
        }
    }

    pub(crate) mod runtime {
        use terminal::Program;
        pub trait Application: Program {
            fn new() -> Self;
            fn should_exit(&self) -> bool;
        }

        pub mod terminal {
            pub trait Program {
                type Msg;
                fn update(&mut self, msg: Self::Msg);
                fn view(&mut self) -> String;
                fn subscribe(&mut self, event: crossterm::event::Event) -> Vec<Self::Msg>;
            }
        }
    }
}

mod ui {
    use super::runtime;
    use super::runtime::Instance;
    pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    pub trait Application {
        type Msg;
        fn new() -> Self;
        fn update(&mut self, msg: Self::Msg);
        fn view(&self) -> String;
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

mod cursor {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Cursor {
        pub row: u16,
        pub col: u16,
    }
}
enum MsgKind {
    Normal(NormalMessage),
}

enum NormalMessage {
    CursorUp,
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
    stdout: std::io::Stdout,
    is_running: bool,
}

impl ui::Application for Revi {
    type Msg = MsgKind;
    fn new() -> Self {
        Self {
            buffers: Vec::new(),
            mode: Mode::Normal,
            cursor: cursor::Cursor::default(),
            stdout: std::io::stdout(),
            is_running: true,
        }
    }

    fn update(&mut self, mode: Self::Msg) {
        match mode {
            MsgKind::Normal(normal) => match normal {
                NormalMessage::CursorUp => self.cursor.row += 1,
            },
        }
    }

    fn should_exit(&self) -> bool {
        self.is_running
    }

    fn view(&self) -> String {
        "Hello World".into()
    }

    fn subscribe(&mut self, event: ui::Event) -> Vec<Self::Msg> {
        let mut messages = Vec::new();
        match event {
            ui::Event::Key(key_event) => match &self.mode {
                Mode::Normal => match key_event {
                    ui::KeyEvent {
                        code: ui::KeyCode::Esc,
                        modifiers: ui::KeyModifiers::NONE,
                    } => {
                        self.is_running = false;
                    }
                    _ => {}
                },
                Mode::Insert => {}
                Mode::Command => {}
            },
            ui::Event::Mouse(_) => {}
            ui::Event::Resize(_, _) => {}
        }
        messages
    }
}
use ui::Application;
fn main() {
    Revi::run();
}
