use crate::key::Key;
use crate::CursorShape;
use crate::Display;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;

#[must_use]
pub fn screen_size() -> (u16, u16) {
    crossterm::terminal::size().expect("Failed to find screen size")
}

#[derive(Debug)]
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

    pub fn update<D: std::fmt::Display>(&mut self, displayable: &mut impl Display<D>) {
        self.save_cursor();
        self.hide_cursor();
        displayable.render(|x, y, text: Vec<D>| {
            self.update_window(x, y, text);
        });
        displayable.cursor(|x, y, shape| {
            self.update_cursor(x, y, shape);
        });
        self.restore_cursor();
        self.show_cursor();
        self.flush();
    }

    fn update_window<D: std::fmt::Display>(&mut self, x: u16, offset_y: u16, text: Vec<D>) {
        for (idx, line) in text.iter().enumerate() {
            let y = offset_y + idx as u16;
            crossterm::queue!(
                self.writer,
                crossterm::cursor::MoveTo(x, y),
                crossterm::style::Print(line),
            )
            .expect("Drawing Window Failed.");
        }
    }

    pub fn update_cursor(&mut self, x: u16, y: u16, shape: Option<CursorShape>) {
        if let Some(shape) = shape {
            self.set_cursor_shape(shape.into());
        }
        crossterm::queue!(
            self.writer,
            crossterm::cursor::RestorePosition,
            crossterm::cursor::MoveTo(x, y),
            crossterm::cursor::SavePosition
        )
        .expect("Failure to update cursor position.");
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

    fn enable_raw_mode() {
        crossterm::terminal::enable_raw_mode().expect("Failure to Enter Raw Mode.")
    }

    fn disable_raw_mode() {
        crossterm::terminal::disable_raw_mode().expect("Failure to Exit Raw Mode.")
    }

    pub fn flush(&mut self) {
        self.writer.flush().expect("Failure to Flush.");
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        Self::disable_raw_mode();
        crossterm::execute!(self.writer, crossterm::terminal::LeaveAlternateScreen)
            .expect("Failure to Leave Alternate Screen.");
    }
}

impl Default for Tui {
    fn default() -> Self {
        let mut writer = stdout();
        crossterm::execute!(&mut writer, crossterm::terminal::EnterAlternateScreen)
            .expect("Failure to Enter Alternate Screen.");
        let tui = Self {
            writer,
            current_event: None,
        };

        Self::enable_raw_mode();

        tui
    }
}
