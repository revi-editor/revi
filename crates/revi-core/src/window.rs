/* windows.rs
*/

use crate::buffer::Buffer;
use crate::line_number::LineNumbers;
use crate::mode::Mode;
use crate::position::Position;
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
    buffer: Buffer,
    /// Name of Text File
    name: Option<String>,
    /// Line Number Type.
    line_number_state: LineNumbers,
}

impl Window {
    pub fn new(width: u16, height: u16, rope: Rope, name: Option<String>) -> Self {
        // TODO: Fix the Starting position of the window.
        let buffer = Buffer::from(rope);
        let line_number_width = (buffer.len_lines().to_string().len().max(3) + 2) as u16;
        Self {
            mode: Mode::Normal,
            dimensions: Position::new_u16(width, height),
            window_offset: Position::default(),
            scroll_offset: Position::default(),
            cursor: Position::default(),
            max_cursor: Position::default(),
            buffer,
            name,
            line_number_state: LineNumbers::AbsoluteNumber(line_number_width),
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
            < self.buffer.len_lines()
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
        } else if self.cursor_file().as_usize_y() < self.buffer.len_lines() - 1 {
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
            } else if self.buffer.len_lines() - 1 == self.cursor_file().as_usize_y() {
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
        self.cursor.set_x(if cols > self.cursor.as_usize_x() {
            0
        } else {
            self.cursor.as_usize_x() - cols
        });
        self.max_cursor.set_x(self.cursor.as_usize_x());
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
        self.cursor.add_to_x(cols);
        self.adjust_cursor_x();
        self.max_cursor.set_x(self.cursor.as_usize_x());
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
        let total_y = self.buffer.len_lines().saturating_sub(1);
        let screen_y = self.height() - 1;
        let offset_y = total_y.saturating_sub(screen_y);
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
        self.max_cursor.set_x(0);
    }

    pub fn end(&mut self) {
        let y = self.cursor_file().as_usize_y();
        self.cursor.set_x(self.buffer.line_len(y));
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
        } else {
            String::new()
        };
        let left = format!(" {} | {}{}", self.mode, self.buffer_name(), debug_line);
        let right = format!(
            "offsets: {} | file: {} | window: {} ",
            self.scroll_offset,
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
        let y = self.position().as_u16_y() + self.height() as u16;
        Position::new_u16(self.position().as_u16_x(), y)
    }

    pub fn line_number(&self) -> String {
        // TODO: Pull this out of Window
        match self.line_number_state {
            LineNumbers::RelativeNumber(_) => "".to_string(),
            LineNumbers::AbsoluteNumber(w) => {
                // TODO: Refactor this to maybe be a trait LineNumers.
                // UI will need to have its own Display trait.
                let file_len = self.buffer.len_lines();
                let max_number_len = file_len.to_string().len();
                let top = self.scroll_offset.as_usize_y();
                let bottom = top + self.height();
                let blank = (0..w + 1)
                    .enumerate()
                    .map(|(i, _)| {
                        if i == 0 {
                            "~"
                        } else if i as u16 == w {
                            "\r\n"
                        } else {
                            " "
                        }
                    })
                    .collect::<String>();
                (top..bottom)
                    .map(|n| {
                        if n > file_len.saturating_sub(1) {
                            return blank.clone();
                        }
                        let padding = (0..(max(max_number_len, w as usize - 2)
                            - n.to_string().len()))
                            .map(|_| " ")
                            .collect::<String>();
                        let line_number = format!(" {}{}", padding, n);
                        let width = (0..w.saturating_sub(line_number.len() as u16))
                            .map(|_| " ")
                            .collect::<String>();
                        format!("{}{}\r\n", line_number, width)
                    })
                    .collect::<String>()
            }
            LineNumbers::None => "".to_string(),
        }
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let top = self.scroll_offset.as_usize_y();
        let bottom = self.dimensions.as_usize_y() + top;
        let window = self.buffer.on_screen(top, bottom);
        let formated_window = format_window_buffer(&window, self.width(), self.height());

        write!(f, "{}", formated_window)
    }
}

fn format_window_buffer(text: &str, width: usize, height: usize) -> String {
    // TODO: Pull this out of window.rs
    let filler = ' '; // std::char::from_u32(9608).unwrap_or('&');
    let mut new = String::new();
    for (y, line) in text.lines().enumerate() {
        if y == height {
            break;
        }
        let l = line.get(..line.len().min(width)).unwrap_or("");
        let w = width.saturating_sub(count_char(l, '\t') * 3);
        let line = line
            .get(..line.len().min(w))
            .unwrap_or("")
            .replace("\t", "    ");
        new.push_str(&line);
        let spaces = width.saturating_sub(line.len());
        let blanks = vec![filler; spaces].iter().collect::<String>();
        new.push_str(&blanks);
        new.push_str("\r\n");
    }
    for _ in 0..(height.saturating_sub(count_char(&new, '\n'))) {
        new.push_str(&vec![filler; width].iter().collect::<String>());
        new.push_str("\r\n");
    }
    new
}

fn count_char(string: &str, chr: char) -> usize {
    // TODO: Pull this out of window.rs
    let mut counter = 0;
    for c in string.chars() {
        if c == chr {
            counter += 1;
        }
    }
    counter
}
