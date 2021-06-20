/* windows.rs
 *
 */

use crate::mode::Mode;
use crate::position::Position;
use ropey::Rope;
use std::fmt;
use std::fs::OpenOptions;
use std::io::BufWriter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    /// Mode Of window
    pub mode: Mode,
    /// Size Of Window
    pub dimensions: Position,
    /// Location of window in Terminal
    pub window_offset: Position,
    /// Location of window in file
    pub scroll_offset: Position,
    /// Cursor Location in File/Window
    pub cursor: Position,
    /// Furthest from 0 the cursor has been.
    pub max_cursor: Position,
    /// Text File Data
    pub buffer: Rope,
    /// Name of Text File
    pub name: Option<String>,

    pub debug: String,
}

impl Window {
    pub fn new(width: u16, height: u16, buffer: Rope, name: Option<String>) -> Self {
        // TODO: Fix the Starting position of the window.
        Self {
            mode: Mode::Normal,
            dimensions: Position::new_u16(width, height),
            window_offset: Position::default(),
            scroll_offset: Position::default(),
            cursor: Position::default(),
            max_cursor: Position::default(),
            name,
            buffer,
            debug: "No Info".to_string(),
        }
    }

    pub fn move_cursor_down(&mut self, lines: usize) -> bool {
        use std::cmp::min;
        let mut scroll = false;
        let limit = self.buffer.len_lines() - 1;
        let target = min(self.cursor.as_usize_y() + lines, limit);
        let diff = target - self.cursor.as_usize_y();

        let max = self.window_offset.as_usize_y() + self.dimensions.as_usize_y();
        if target >= max {
            self.scroll_offset.add_to_y(target - max + 1);
            scroll = true;
        } else {
            self.cursor.add_to_y(diff);
        }
        self.cursor.set_x(self.max_cursor.as_usize_x());
        self.adjust_cursor_x();
        scroll
    }

    pub fn move_cursor_up(&mut self, lines: usize) -> bool {
        let mut scroll = false;
        let target = if lines > self.cursor.as_usize_y() {
            0
        } else {
            self.cursor.as_usize_y() - lines
        };
        // let target = self.cursor.as_usize_y().saturating_sub(lines);

        let min = self.window_offset.as_usize_y();
        // FIXME: scrolls on second line instead of first.
        if target == min && self.cursor_screen().as_usize_y() == 0 {
            let scroll_offset = self
                .scroll_offset
                .as_usize_y()
                .saturating_sub(target)
                .saturating_sub(1);
            self.scroll_offset.set_y(scroll_offset);
            scroll = true;
        }

        self.cursor = Position::new(self.max_cursor.as_usize_x(), target);
        self.adjust_cursor_x();
        scroll
    }

    fn adjust_cursor_x(&mut self) {
        use std::cmp::min;
        let line = self
            .buffer
            .line(self.cursor.as_usize_y() + self.scroll_offset.as_usize_y());
        let line_len = line.len_chars();
        // if line_len > 0 {
        //     line_len -= 1;
        // }
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

    pub fn move_cursor_right(&mut self, cols: usize) {
        self.cursor.add_to_x(cols);
        self.adjust_cursor_x();
        self.max_cursor.set_x(self.cursor.as_usize_x());
    }

    pub fn insert_enter(&mut self) {
        self.insert_char('\n');
        self.move_cursor_down(1);
        self.cursor.set_x(0);
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

    pub fn buffer_name(&self) -> String {
        self.name.clone().unwrap_or("UNNAMED".to_string())
    }

    pub fn cursor_file(&self) -> Position {
        self.cursor + self.scroll_offset
    }

    pub fn cursor_screen(&self) -> Position {
        self.cursor + self.window_offset
    }

    pub fn save(&self) {
        let file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open(self.buffer_name())
            .expect("Problem opening the file for saving");

        let buff = BufWriter::new(file);
        self.buffer
            .write_to(buff)
            .expect("Failed to write to File buffer to file");
    }

    pub fn status_bar(&self) -> String {
        let left = format!(" {} | {} ", self.mode, self.buffer_name());
        let right = format!(" {} | {} ", self.cursor_screen(), self.cursor_file());
        let middle = (0..left.len() + right.len())
            .map(|_| " ")
            .collect::<String>();
        format!("{}{}{}", left, right, middle)
    }

    pub fn status_bar_pos(&self) -> Position {
        let y = (self.window_offset + self.dimensions).as_u16_y();
        Position::new_u16(self.window_offset.as_u16_x(), y)
    }

    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.cursor_file().as_usize();
        let line_index = self.buffer.line_to_char(y);
        self.buffer.insert_char(line_index + x, c);
        self.move_cursor_right(1);
    }
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /* Formatting Screen Instructions
         *
         * 1. Get the text from buffer relevant to screen size
         *      and location of view into file.
         *
         * 2. Give string to format_window_buffer function with
         *      the dimensions of the Window.
         *
         * 3. Give output to write macro.
         */

        // Step 1
        let top_line = self.scroll_offset.as_usize_y();
        let bottom_line = self.dimensions.as_usize_y() + top_line; // This may need to be a variable.
        let window = self
            .buffer
            .lines_at(top_line)
            .enumerate()
            .filter(|(i, _)| *i < bottom_line)
            .map(|(_, s)| s.to_string())
            .collect::<String>();
        let (w, h) = self.dimensions.as_usize();
        let formated_window = format_window_buffer(&window, w, h);

        write!(f, "{}", formated_window)
    }
}

fn format_window_buffer(text: &str, width: usize, height: usize) -> String {
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

fn _format_command_bar(line: &mut String, length: usize) {
    let filler = ' ';
    let spaces = length.saturating_sub(line.len());
    let blanks = vec![filler; spaces].iter().collect::<String>();
    line.push_str(&blanks);
}

fn count_char(string: &str, chr: char) -> usize {
    let mut counter = 0;
    for c in string.chars() {
        if c == chr {
            counter += 1;
        }
    }
    counter
}
