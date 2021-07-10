/* windows.rs
*/

use crate::buffer::Buffer;
use crate::line_number::LineNumbers;
use crate::mode::Mode;
use crate::position::Position;
// use crate::text_formater::format_window_buffer;
use crate::text_formater::format_screen;
use ropey::Rope;
use std::cmp::{max, min};
use std::fmt;
use std::io::BufWriter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    /// Mode Of window
    pub mode: Mode,
    /// Size Of Window
    pub dimensions: Position,
    /// Location of window in Terminal
    window_offset: Position,
    /// Location of window in file
    scroll_offset: Position,
    /// Cursor Location in File/Window
    cursor: Position,
    /// Furthest from 0 the cursor has been.
    max_cursor: Position,
    /// Text File Data
    buffer: Buffer, //  Move buffer into a list
    /// Name of Text File
    name: Option<String>, // Make this from a String to a Path
    /// Line Number Type.
    line_number_state: LineNumbers,
}

impl Window {
    pub fn new(width: u16, height: u16, rope: Rope, name: Option<String>) -> Self {
        // TODO: Fix the Starting position of the window.
        let buffer = Buffer::from(rope);
        let line_number_width = buffer.len_lines().to_string().len().max(3) + 2;
        Self {
            mode: Mode::Normal,
            dimensions: Position::new_u16(width, height - 1),
            window_offset: Position::default(),
            scroll_offset: Position::default(),
            cursor: Position::default(),
            max_cursor: Position::default(),
            buffer,
            name,
            line_number_state: LineNumbers::RelativeNumber(line_number_width, height as usize - 1),
        }
    }

    pub fn position(&self) -> Position {
        self.window_offset
    }

    pub fn offset(&self) -> Position {
        self.window_offset + Position::new(self.line_number_state.width(), 0)
    }

    pub fn set_cursor(&mut self, pos: Position) {
        self.cursor = pos;
    }

    pub fn height(&self) -> usize {
        self.dimensions.as_usize_y()
    }

    pub fn width(&self) -> usize {
        self.dimensions
            .as_usize_x()
            .saturating_sub(self.line_number_state.width())
    }

    pub fn cursor_file(&self) -> Position {
        self.cursor + self.scroll_offset
    }

    pub fn cursor_screen(&self) -> Position {
        self.cursor + self.offset()
    }

    pub fn scroll_down(&mut self, lines: usize) {
        if lines + self.scroll_offset.as_usize_y() + self.cursor.as_usize_y()
            < self.buffer.len_lines().saturating_sub(1)
        {
            self.scroll_offset.add_to_y(lines);
            self.adjust_cursor_x()
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset.sub_to_y(lines);
        self.adjust_cursor_x()
    }

    pub fn move_cursor_down(&mut self, lines: usize) {
        if self.cursor.as_usize_y() >= self.height() - 1 {
            self.scroll_down(lines);
        } else if self.cursor_file().as_usize_y() <= self.buffer.len_lines().saturating_sub(1) {
            self.cursor.add_to_y(lines);
            self.cursor.set_x(self.max_cursor.as_usize_x());
            self.adjust_cursor_x()
        }
    }

    pub fn move_cursor_up(&mut self, lines: usize) {
        if self.cursor.as_usize_y() == 0 {
            self.scroll_up(lines);
        } else {
            self.cursor.sub_to_y(lines);
            self.cursor.set_x(self.max_cursor.as_usize_x());
            self.adjust_cursor_x()
        }
    }

    pub fn adjust_cursor_x(&mut self) {
        let line = self
            .buffer
            .line(self.cursor_file().as_usize_y())
            .chars()
            .collect::<String>();
        let mut line_len = line.len();
        if let Mode::Normal = self.mode {
            if line.ends_with('\n') {
                line_len = line_len.saturating_sub(2);
            } else if self.buffer.len_lines().saturating_sub(1) == self.cursor_file().as_usize_y() {
                line_len = line_len.saturating_sub(1);
            }
        } else if let Mode::Insert = self.mode {
            if line.ends_with('\n') {
                line_len = line_len.saturating_sub(1);
            }
        }

        self.cursor.set_x(min(line_len, self.cursor.as_usize_x()));
    }

    pub fn move_cursor_left(&mut self, cols: usize) {
        if self.cursor.as_usize_x() == 0 {
            self.scroll_left(cols);
        } else {
            self.cursor.set_x(if cols > self.cursor.as_usize_x() {
                0
            } else {
                self.cursor.as_usize_x() - cols
            });
            self.max_cursor.set_x(self.cursor.as_usize_x());
        }
    }

    pub fn scroll_left(&mut self, cols: usize) {
        self.scroll_offset.sub_to_x(cols);
    }

    pub fn move_forward_by_word(&mut self) {
        let pos = self.cursor + self.scroll_offset;
        if let Some(i) = self.buffer.next_jump_idx(&pos) {
            self.cursor.set_x(i);
            self.max_cursor.set_x(self.cursor.as_usize_x());
        } else {
            self.move_cursor_down(1);
            self.first_char_in_line();
        }
    }

    pub fn move_backward_by_word(&mut self) {
        let pos = self.cursor + self.scroll_offset;
        if pos == Position::new(0, 0) {
            return;
        }
        if let Some(i) = self.buffer.prev_jump_idx(&pos) {
            self.cursor.set_x(i);
            self.max_cursor.set_x(self.cursor.as_usize_x());
        } else {
            self.move_cursor_up(1);
            self.end();
            let pos = self.cursor + self.scroll_offset;
            if let Some(i) = self.buffer.prev_jump_idx(&pos) {
                self.cursor.set_x(i);
                self.max_cursor.set_x(self.cursor.as_usize_x());
            }
        }
    }

    pub fn move_cursor_right(&mut self, cols: usize) {
        if self.cursor.as_usize_x() >= self.width() - 1 {
            self.scroll_right(cols)
        } else {
            self.cursor.add_to_x(cols);
            self.max_cursor.set_x(self.cursor.as_usize_x());
            self.adjust_cursor_x();
        }
    }

    pub fn scroll_right(&mut self, cols: usize) {
        if cols + self.scroll_offset.as_usize_x() + self.cursor.as_usize_x()
            < self.buffer.line_len(self.cursor_file().as_usize_y())
        {
            self.scroll_offset.add_to_x(cols);
            // self.adjust_cursor_x()
        }
    }

    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
        self.move_cursor_down(1);
        self.cursor.set_x(0);
    }

    pub fn first_char_in_line(&mut self) {
        let y = (self.cursor + self.scroll_offset).as_usize_y();
        for (i, c) in self.buffer.line(y).chars().enumerate() {
            if c != ' ' {
                self.cursor.set_x(i);
                self.max_cursor.set_x(max(i, self.cursor.as_usize_x()));
                break;
            }
        }
    }

    pub fn jump_to_first_line_buffer(&mut self) {
        self.cursor.set_y(0);
        self.scroll_offset.set_y(0);
        self.adjust_cursor_x()
    }

    pub fn jump_to_last_line_buffer(&mut self) {
        // Gets line count but screen is off by one so we subtract one.
        let total_y = self.buffer.len_lines().saturating_sub(1);
        // Gets screen height but it also is off by one so we subtract one.
        let screen_y = self.height() - 1;
        // Finds Y offset into file but it is off by one as well for indexing so we subtract one as
        // well
        let offset_y = total_y.saturating_sub(screen_y).saturating_sub(1);
        self.cursor.set_y(screen_y);
        self.scroll_offset.set_y(offset_y);
        self.adjust_cursor_x()
    }

    pub fn backspace(&mut self) {
        if self.cursor_file().as_u16() == (0, 0) {
            return;
        }

        let line_index = self.buffer.line_to_char(self.cursor_file().as_usize_y());
        let index = line_index + self.cursor_file().as_usize_x() - 1;
        self.buffer.remove(index..index + 1);

        let new_line = self.buffer.char_to_line(index);
        if new_line != self.cursor_file().as_usize_y() {
            self.move_cursor_up(1);
        }
        self.cursor
            .set_x(index - self.buffer.line_to_char(new_line));
    }

    pub fn delete(&mut self) {
        let index = self.buffer.line_to_char(self.cursor_file().as_usize_y())
            + self.cursor_file().as_usize_x();
        if index < self.buffer.len_chars() {
            self.buffer.remove(index..index + 1);
        }
        self.adjust_cursor_x();
    }

    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.cursor_file().as_usize();
        let line_index = self.buffer.line_to_char(y);
        self.buffer.insert_char(line_index + x, c);
        self.move_cursor_right(1);
    }

    pub fn delete_line(&mut self) {
        let y = self.cursor_file().as_usize_y();
        let start_idx = self.buffer.line_to_char(y);
        let end_idx = self.buffer.line_to_char(y + 1);

        // Remove the line...
        self.buffer.remove(start_idx..end_idx);
        self.adjust_cursor_x();
    }

    pub fn home(&mut self) {
        self.cursor.set_x(0);
        self.scroll_offset.set_x(0);
        self.max_cursor.set_x(0);
    }

    pub fn end(&mut self) {
        let y = self.cursor_file().as_usize_y();
        let line_len = self.buffer.line_len(y);
        let cursor = line_len.min(self.width() - 1);
        let offset = if line_len >= self.width() - 1 {
            line_len.saturating_sub(cursor)
        } else {
            0
        };
        self.cursor.set_x(cursor);
        self.scroll_offset.set_x(offset);
        self.max_cursor.set_x(self.cursor.as_usize_x());
        self.adjust_cursor_x();
    }

    pub fn buffer_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "UNNAMED".to_string())
    }

    pub fn save(&self) {
        let file =
            std::fs::File::create(self.buffer_name()).expect("Problem opening the file for saving");

        let buff = BufWriter::new(file);
        self.buffer
            .write_to(buff)
            .expect("Failed to write to file.");
    }

    pub fn status_bar(&self) -> String {
        // TODO: Pull this out of Window
        let debug_line = if cfg!(feature = "debug_line") {
            format!(
                " | {:?}",
                self.buffer
                    .line(self.cursor_file().as_usize_y())
                    .chars()
                    .collect::<String>()
            )
        } else if cfg!(feature = "debug_line_words") {
            let pos = self.cursor + self.scroll_offset;
            let left = "NOTHING";
            let right = self.buffer.next_jump_idx(&pos);
            format!(" | {:?}<{}>{:?}", left, pos.as_u16_x(), right)
        } else if cfg!(feature = "debug_offset") {
            format!(" | {}", self.scroll_offset)
        } else {
            String::new()
        };
        let left = format!(" {} | {}{}", self.mode, self.buffer_name(), debug_line);
        let right = format!(
            "file: {} | window: {} ",
            self.scroll_offset + self.cursor,
            self.window_offset + self.cursor
        );
        let middle = (0..(self.width().saturating_sub(left.len() + right.len())))
            .map(|_| ' ')
            .collect::<String>();
        format!("{}{}{}", left, middle, right)
    }

    pub fn status_bar_pos(&self) -> Position {
        // TODO: Pull this out of Window
        let y = self.position().as_usize_y() + self.height();
        Position::new(self.position().as_usize_x(), y)
    }

    pub fn line_number(&self) -> String {
        // TODO: Pull this out of Window
        let scroll = self.scroll_offset.as_usize_y();
        let len_lines = self.buffer.len_lines().saturating_sub(1);
        let y = self
            .cursor_file()
            .as_usize_y()
            .saturating_sub(self.scroll_offset.as_usize_y());
        self.line_number_state.lines(scroll, len_lines, y)
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let top = self.scroll_offset.as_usize_y();
        let bottom = self.dimensions.as_usize_y() + top;
        let window = self.buffer.on_screen(top, bottom);
        let formated_window = format_screen(
            &window,
            self.scroll_offset.as_usize_x(),
            self.width(),
            self.height(),
        );

        write!(f, "{}", formated_window)
    }
}
