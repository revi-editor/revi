/* windows.rs
*/

use crate::buffer::Buffer;
use crate::line_number::LineNumbers;
use crate::mode::Mode;
use crate::position::Position;
use crate::text_formater::format_screen;
use std::cmp::{max, min};
use std::io::BufWriter;

#[derive(Debug)]
pub struct Window {
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
    buffer: Option<usize>,
    /// Line Number Type.
    line_number_type: LineNumbers,
    /// This needs to be removed
    status_bar_state: bool,
}

impl Window {
    pub fn new(width: u16, height: u16, buffer: Option<usize>) -> Self {
        Self {
            mode: Mode::Normal,
            dimensions: Position::new_u16(width, height),
            window_offset: Position::default(),
            scroll_offset: Position::default(),
            cursor: Position::default(),
            max_cursor: Position::default(),
            buffer,
            line_number_type: LineNumbers::None,
            status_bar_state: false,
        }
    }

    pub fn with_position(mut self, pos: Position) -> Self {
        self.window_offset = pos;
        self
    }

    pub fn with_line_numbers(mut self, _type: LineNumbers) -> Self {
        self.line_number_type = _type;
        self
    }

    pub fn with_status_bar(mut self, state: bool) -> Self {
        self.dimensions.sub_to_y(1);
        self.status_bar_state = state;
        self
    }

    pub fn set_buffer(&mut self, buffer: Option<usize>) {
        self.scroll_offset = Position::default();
        self.cursor = Position::default();
        self.max_cursor = Position::default();
        self.buffer = buffer;
    }

    pub fn get_buffer_id(&self) -> Option<usize> {
        self.buffer
    }

    pub fn position(&self) -> Position {
        self.window_offset
    }

    pub fn offset(&self, buffer: &Buffer) -> Position {
        self.window_offset + Position::new(self.line_number_width(buffer), 0)
    }

    pub fn set_cursor(&mut self, pos: Position) {
        self.cursor = pos;
    }

    pub fn height(&self) -> usize {
        self.dimensions.as_usize_y()
    }

    pub fn width(&self, buffer: &Buffer) -> usize {
        self.dimensions
            .as_usize_x()
            .saturating_sub(self.line_number_width(buffer))
    }

    pub fn cursor_file(&self) -> Position {
        self.cursor + self.scroll_offset
    }

    pub fn cursor_screen(&self, buffer: &Buffer) -> Position {
        self.cursor + self.offset(buffer)
    }

    pub fn scroll_down(&mut self, lines: usize, buffer: &Buffer) {
        if lines + self.scroll_offset.as_usize_y() + self.cursor.as_usize_y()
            < buffer.len_lines().saturating_sub(1)
        {
            self.scroll_offset.add_to_y(lines);
            self.adjust_cursor_x(buffer);
        }
    }

    pub fn scroll_up(&mut self, lines: usize, buffer: &Buffer) {
        self.scroll_offset.sub_to_y(lines);
        self.adjust_cursor_x(buffer)
    }

    pub fn move_cursor_down(&mut self, lines: usize, buffer: &Buffer) {
        if self.cursor.as_usize_y() >= self.height() - 1 {
            self.scroll_down(lines, buffer);
        } else if self.cursor_file().as_usize_y() <= buffer.len_lines().saturating_sub(1) {
            self.cursor.add_to_y(lines);
            self.cursor.set_x(self.max_cursor.as_usize_x());
            self.adjust_cursor_x(buffer)
        }
    }

    pub fn move_cursor_up(&mut self, lines: usize, buffer: &Buffer) {
        if self.cursor.as_usize_y() == 0 {
            self.scroll_up(lines, buffer);
        } else {
            self.cursor.sub_to_y(lines);
            self.cursor.set_x(self.max_cursor.as_usize_x());
            self.adjust_cursor_x(buffer)
        }
    }

    pub fn adjust_cursor_x(&mut self, buffer: &Buffer) {
        let line = buffer
            .line(self.cursor_file().as_usize_y())
            .chars()
            .collect::<String>();
        let mut line_len = line.len();
        if let Mode::Normal = self.mode {
            if line.ends_with('\n') {
                line_len = line_len.saturating_sub(2);
            } else if buffer.len_lines().saturating_sub(1) == self.cursor_file().as_usize_y() {
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

    pub fn move_forward_by_word(&mut self, buffer: &Buffer) {
        let pos = self.cursor + self.scroll_offset;
        if let Some(i) = buffer.next_jump_idx(&pos) {
            self.cursor.set_x(i);
            self.max_cursor.set_x(self.cursor.as_usize_x());
        } else {
            self.move_cursor_down(1, buffer);
            self.first_char_in_line(buffer);
        }
    }

    pub fn move_backward_by_word(&mut self, buffer: &Buffer) {
        let pos = self.cursor + self.scroll_offset;
        if pos == Position::new(0, 0) {
            return;
        }
        if let Some(i) = buffer.prev_jump_idx(&pos) {
            self.cursor.set_x(i);
            self.max_cursor.set_x(self.cursor.as_usize_x());
        } else {
            self.move_cursor_up(1, buffer);
            self.end(buffer);
            let pos = self.cursor + self.scroll_offset;
            if let Some(i) = buffer.prev_jump_idx(&pos) {
                self.cursor.set_x(i);
                self.max_cursor.set_x(self.cursor.as_usize_x());
            }
        }
    }

    pub fn move_cursor_right(&mut self, cols: usize, buffer: &Buffer) {
        if self.cursor.as_usize_x() >= self.width(buffer) - 1 {
            self.scroll_right(cols, buffer)
        } else {
            self.cursor.add_to_x(cols);
            self.max_cursor.set_x(self.cursor.as_usize_x());
            self.adjust_cursor_x(buffer);
        }
    }

    pub fn scroll_right(&mut self, cols: usize, buffer: &Buffer) {
        if cols + self.scroll_offset.as_usize_x() + self.cursor.as_usize_x()
            < buffer.line_len(self.cursor_file().as_usize_y())
        {
            self.scroll_offset.add_to_x(cols);
            // self.adjust_cursor_x()
        }
    }

    pub fn insert_newline(&mut self, buffer: &mut Buffer) {
        self.insert_char('\n', buffer);
        self.move_cursor_down(1, buffer);
        self.cursor.set_x(0);
    }

    pub fn first_char_in_line(&mut self, buffer: &Buffer) {
        let y = (self.cursor + self.scroll_offset).as_usize_y();
        for (i, c) in buffer.line(y).chars().enumerate() {
            if c != ' ' {
                self.cursor.set_x(i);
                self.max_cursor.set_x(max(i, self.cursor.as_usize_x()));
                break;
            }
        }
    }

    pub fn jump_to_first_line_buffer(&mut self, buffer: &Buffer) {
        self.cursor.set_y(0);
        self.scroll_offset.set_y(0);
        self.adjust_cursor_x(buffer)
    }

    pub fn jump_to_last_line_buffer(&mut self, buffer: &Buffer) {
        // Gets line count but screen is off by one so we subtract one.
        let total_y = buffer.len_lines().saturating_sub(1);
        // Gets screen height but it also is off by one so we subtract one.
        let screen_y = self.height() - 1;
        // Finds Y offset into file but it is off by one as well for indexing so we subtract one as
        // well
        let offset_y = total_y.saturating_sub(screen_y).saturating_sub(1);
        self.cursor.set_y(screen_y);
        self.scroll_offset.set_y(offset_y);
        self.adjust_cursor_x(buffer)
    }

    pub fn backspace(&mut self, buffer: &mut Buffer) {
        if self.cursor_file().as_u16() == (0, 0) {
            return;
        }

        let line_index = buffer.line_to_char(self.cursor_file().as_usize_y());
        let index = line_index + self.cursor_file().as_usize_x() - 1;
        buffer.remove(index..index + 1);

        let new_line = buffer.char_to_line(index);
        if new_line != self.cursor_file().as_usize_y() {
            self.move_cursor_up(1, buffer);
        }
        self.cursor.set_x(index - buffer.line_to_char(new_line));
    }

    pub fn delete(&mut self, buffer: &mut Buffer) {
        let index =
            buffer.line_to_char(self.cursor_file().as_usize_y()) + self.cursor_file().as_usize_x();
        if index < buffer.len_chars() {
            buffer.remove(index..index + 1);
        }
        self.adjust_cursor_x(buffer);
    }

    pub fn insert_char(&mut self, c: char, buffer: &mut Buffer) {
        let (x, y) = self.cursor_file().as_usize();
        let line_index = buffer.line_to_char(y);
        buffer.insert_char(line_index + x, c);
        self.move_cursor_right(1, buffer);
    }

    pub fn delete_line(&mut self, buffer: &mut Buffer) {
        let y = self.cursor_file().as_usize_y();
        let start_idx = buffer.line_to_char(y);
        let end_idx = buffer.line_to_char(y + 1);

        // Remove the line...
        buffer.remove(start_idx..end_idx);
        self.adjust_cursor_x(buffer);
    }

    pub fn home(&mut self) {
        self.cursor.set_x(0);
        self.scroll_offset.set_x(0);
        self.max_cursor.set_x(0);
    }

    pub fn end(&mut self, buffer: &Buffer) {
        let y = self.cursor_file().as_usize_y();
        let line_len = buffer.line_len(y);
        let cursor = line_len.min(self.width(buffer) - 1);
        let offset = if line_len >= self.width(buffer) - 1 {
            line_len.saturating_sub(cursor)
        } else {
            0
        };
        self.cursor.set_x(cursor);
        self.scroll_offset.set_x(offset);
        self.max_cursor.set_x(self.cursor.as_usize_x());
        self.adjust_cursor_x(buffer);
    }

    // This need to return a Result
    pub fn save(&self, buffer: &Buffer) {
        if let Some(filename) = buffer.name() {
            let file =
                std::fs::File::create(filename).expect("Problem opening the file for saving");

            let buff = BufWriter::new(file);
            buffer.write_to(buff).expect("Failed to write to file.");
        }
    }

    pub fn get_status_bar(&self, buffer: &Buffer) -> Option<((u16, u16), String)> {
        // FIXME: I hate this so much
        if self.status_bar_state {
            let y = self.position().as_usize_y() + self.height();
            let pos = Position::new(self.position().as_usize_x(), y);

            let left = format!(
                " {} | {}",
                self.mode,
                buffer.name().clone().unwrap_or("N/A".to_string())
            );
            let right = format!(
                "file: {} | window: {} ",
                self.cursor_file(),
                self.cursor_screen(buffer),
            );
            let middle = (0..(self.width(buffer).saturating_sub(left.len() + right.len())))
                .map(|_| ' ')
                .collect::<String>();
            return Some((pos.as_u16(), format!("{}{}{}", left, middle, right)));
        }
        None
    }

    pub fn get_line_number(&self, buffer: &Buffer) -> Option<((u16, u16), String)> {
        // TODO: Pull this out of Window
        if self.line_number_type != LineNumbers::None {
            let scroll = self.scroll_offset.as_usize_y();
            let len_lines = buffer.len_lines().saturating_sub(1);
            let y = self
                .cursor_file()
                .as_usize_y()
                .saturating_sub(self.scroll_offset.as_usize_y());
            let width = self.line_number_width(buffer);
            return Some((
                self.position().as_u16(),
                self.line_number_type
                    .lines(width, self.height(), scroll, len_lines, y),
            ));
        }
        None
    }
}

// Private Methods
impl Window {
    /// Width of LineNumber Area if populated.
    fn line_number_width(&self, buffer: &Buffer) -> usize {
        if self.line_number_type != LineNumbers::None {
            buffer.len_lines().to_string().len().max(3) + 2
        } else {
            0
        }
    }
}

impl revi_ui::Display<Buffer> for Window {
    fn render<F: FnMut(u16, u16, String)>(&self, buffer: &Buffer, mut func: F) {
        let top = self.scroll_offset.as_usize_y();
        let bottom = self.dimensions.as_usize_y() + top;
        let window = buffer.on_screen(top, bottom);
        let formated_window = format_screen(
            &window,
            self.scroll_offset.as_usize_x(),
            self.width(buffer),
            self.height(),
        );
        let (x, y) = self.offset(buffer).as_u16();

        func(x, y, formated_window);
    }
    fn line_numbers<F: FnMut(u16, u16, String)>(&self, buffer: &Buffer, mut func: F) {
        if let Some(((x, y), text)) = self.get_line_number(buffer) {
            func(x, y, text);
        }
    }

    fn status_bar<F: FnMut(u16, u16, String)>(&self, buffer: &Buffer, mut func: F) {
        if let Some(((x, y), text)) = self.get_status_bar(buffer) {
            func(x, y, text);
        }
    }

    fn cursor<F: FnMut(u16, u16, Option<revi_ui::CursorShape>)>(
        &self,
        buffer: &Buffer,
        mut func: F,
    ) {
        let (x, y) = self.cursor_screen(buffer).as_u16();
        func(x, y, Some(self.mode.shape()));
    }
}
