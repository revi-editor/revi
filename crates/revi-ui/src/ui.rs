use crate::key::Key;
use std::fmt::{self, Debug};
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

pub fn screen_size() -> (u16, u16) {
    crossterm::terminal::size().expect("Failed to find screen size")
}

#[derive(Clone)]
pub enum Render {
    Window { pos: (u16, u16), text: String },
    StatusBar { pos: (u16, u16), text: String },
    LineNumbers { pos: (u16, u16), text: String },
    Cursor((u16, u16)),
    CursorShapeBlock,
    CursorShapeLine,
}

impl fmt::Debug for Render {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output;
        match self {
            Self::Window { pos, .. } => output = format!("Window({}, {})", pos.0, pos.1),
            Self::StatusBar { pos, .. } => output = format!("StatusBar({}, {})", pos.0, pos.1),
            Self::LineNumbers { pos, .. } => output = format!("LineNumbers({}, {})", pos.0, pos.1),
            Self::Cursor(pos) => output = format!("Cursor({}, {})", pos.0, pos.1),
            Self::CursorShapeBlock => output = "CursorShapeBlock".to_string(),
            Self::CursorShapeLine => output = "CursorShapeLine".to_string(),
        }
        write!(f, "{}", output)
    }
}

pub struct Tui {
    writer: Stdout,
    current_event: Option<crossterm::event::Event>,
}

impl Tui {
    pub fn poll_read(&mut self, time: Duration) -> bool {
        if crossterm::event::poll(time).expect("Failed to poll.") {
            self.current_event =
                Some(crossterm::event::read().expect("Failure to read crossterm Event."));
            return true;
        }
        self.current_event = None;
        false
    }

    pub fn get_key_press(&mut self) -> (Key, Key) {
        let mut keys = (Key::Null, Key::Null);
        if let Some(crossterm::event::Event::Key(key_event)) = self.current_event {
            keys = Key::from_event(key_event);
        }
        self.current_event = None;
        keys
    }

    pub fn update(&mut self, render: &[Render]) {
        if render.is_empty() {
            return;
        }
        self.save_cursor();
        self.hide_cursor();
        if cfg!(feature = "debug_bar") {
            self.debug(render);
        }
        for revent in render.iter() {
            match revent {
                Render::Cursor(pos) => self.update_cursor(pos),
                Render::StatusBar { pos, text } => self.update_status_bar(pos, text),
                Render::Window { pos, text } => self.update_window(pos, text), // Possible rename to redaw_area
                Render::LineNumbers { pos, text } => self.update_window(pos, text),
                Render::CursorShapeBlock => {
                    self.set_cursor_shape(crossterm::cursor::CursorShape::Block)
                }
                Render::CursorShapeLine => {
                    self.set_cursor_shape(crossterm::cursor::CursorShape::Line)
                }
            }
        }
        self.restore_cursor();
        self.show_cursor();
        self.flush();
    }

    pub fn debug<T>(&mut self, t: T)
    where
        T: Debug,
    {
        crossterm::queue!(
            self.writer,
            crossterm::cursor::MoveTo(0, 10000),
            crossterm::style::Print(format!(
                "{:?}                                                ",
                t
            )),
        )
        .expect("Printing Debug Failed.");
    }

    fn update_window(&mut self, pos: &(u16, u16), text: &str) {
        let offset_y = pos.1;
        for (idx, line) in text.to_string().lines().enumerate() {
            let y = offset_y + idx as u16;
            crossterm::queue!(
                self.writer,
                crossterm::cursor::MoveTo(pos.0, y),
                crossterm::style::Print(line.strip_suffix("\r\n").unwrap_or(line)),
            )
            .expect("Drawing Window Failed.");
        }
    }

    pub fn update_status_bar(&mut self, pos: &(u16, u16), text: &str) {
        crossterm::queue!(
            self.writer,
            crossterm::cursor::MoveTo(pos.0, pos.1),
            crossterm::style::Print(text),
        )
        .expect("Drawing StatusBar Failed.");
    }

    pub fn _update_command_bar(&mut self) {}

    pub fn update_cursor(&mut self, pos: &(u16, u16)) {
        crossterm::queue!(
            self.writer,
            crossterm::cursor::RestorePosition,
            crossterm::cursor::MoveTo(pos.0, pos.1),
            crossterm::cursor::SavePosition
        )
        .expect("Failure to update cursor position.");
    }

    pub fn _update_windows<T>(&mut self, windows: &[T], positions: &[(u16, u16)])
    where
        T: std::fmt::Display,
    {
        self.save_cursor();
        self.hide_cursor();
        for (window, (x, y)) in itertools::izip!(windows, positions) {
            for (idx, line) in window.to_string().lines().enumerate() {
                let y = y + idx as u16;
                crossterm::queue!(
                    self.writer,
                    crossterm::cursor::MoveTo(*x, y),
                    crossterm::style::Print(line),
                )
                .expect("Drawing Window Failed.");
            }
        }
        self.restore_cursor();
        self.show_cursor();
        self.flush();
    }

    fn set_cursor_shape(&mut self, shape: crossterm::cursor::CursorShape) {
        crossterm::queue!(self.writer, crossterm::cursor::SetCursorShape(shape),)
            .expect("Failure to Save Cursor Position.");
    }

    fn save_cursor(&mut self) {
        crossterm::queue!(self.writer, crossterm::cursor::SavePosition,)
            .expect("Failure to Save Cursor Position.");
    }

    fn restore_cursor(&mut self) {
        crossterm::queue!(self.writer, crossterm::cursor::RestorePosition,)
            .expect("Failure to Restore Cursor Position.");
    }

    fn hide_cursor(&mut self) {
        crossterm::queue!(self.writer, crossterm::cursor::Hide,)
            .expect("Failure to Hide Cursor Position.");
    }

    fn show_cursor(&mut self) {
        crossterm::queue!(self.writer, crossterm::cursor::Show,)
            .expect("Failure to Show Cursor Position.");
    }

    fn enable_raw_mode(&mut self) {
        crossterm::terminal::enable_raw_mode().expect("Failure to Enter Raw Mode.")
    }

    fn disable_raw_mode(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failure to Exit Raw Mode.")
    }

    pub fn flush(&mut self) {
        self.writer.flush().expect("Failure to Flush.");
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.disable_raw_mode();
        crossterm::execute!(self.writer, crossterm::terminal::LeaveAlternateScreen)
            .expect("Failure to Leave Alternate Screen.");
    }
}

impl Default for Tui {
    fn default() -> Self {
        let mut writer = stdout();
        crossterm::execute!(&mut writer, crossterm::terminal::EnterAlternateScreen)
            .expect("Failure to Enter Alternate Screen.");
        let mut tui = Self {
            writer,
            current_event: None,
        };
        tui.enable_raw_mode();
        tui
    }
}
