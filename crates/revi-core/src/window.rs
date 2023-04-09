/* windows.rs
*/
#![allow(unused)]

use crate::buffer::Buffer;
use crate::line_number::{LineNumberBuilder, LineNumberKind};
use crate::mode::Mode;
use crate::position::Position;
use crate::text_formater::format_screen;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::{max, min};
use std::io::BufWriter;
use std::rc::Rc;

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
    buffer: Rc<RefCell<Buffer>>,
    /// Line Number Type.
    line_number_type: LineNumberKind,
    /// This needs to be removed
    status_bar_state: bool,
}

impl Window {
    pub fn new(width: u16, height: u16, buffer: Rc<RefCell<Buffer>>) -> Self {
        Self {
            mode: Mode::Normal,
            dimensions: Position::new_u16(width, height),
            window_offset: Position::default(),
            scroll_offset: Position::default(),
            cursor: Position::default(),
            max_cursor: Position::default(),
            buffer,
            line_number_type: LineNumberKind::None,
            status_bar_state: false,
        }
    }

    #[must_use]
    pub fn with_position(mut self, pos: Position) -> Self {
        self.window_offset = pos;
        self
    }

    #[must_use]
    pub fn with_line_numbers(mut self, line_number_type: LineNumberKind) -> Self {
        self.line_number_type = line_number_type;
        self
    }

    #[must_use]
    pub fn with_status_bar(mut self, state: bool) -> Self {
        self.dimensions.sub_to_y(1);
        self.status_bar_state = state;
        self
    }

    pub fn set_buffer(&mut self, buffer: Rc<RefCell<Buffer>>) {
        self.scroll_offset = Position::default();
        self.cursor = Position::default();
        self.max_cursor = Position::default();
        self.buffer = buffer;
    }

    pub fn set_number(&mut self, number_type: LineNumberKind) {
        self.line_number_type = number_type;
    }

    /// Gives a borrowed copy of the buffer.
    #[must_use]
    pub fn buffer(&self) -> Ref<Buffer> {
        self.buffer.borrow()
    }

    #[must_use]
    pub fn buffer_mut(&mut self) -> RefMut<Buffer> {
        self.buffer.borrow_mut()
    }

    #[must_use]
    pub fn dimensions(&self) -> Position {
        self.dimensions
    }

    #[must_use]
    pub fn position(&self) -> Position {
        self.window_offset
    }

    #[must_use]
    pub fn offset(&self) -> Position {
        self.window_offset + Position::new(self.line_number_width(), 0)
    }

    pub fn set_cursor(&mut self, pos: Position) {
        self.cursor = pos;
    }

    pub fn goto(&mut self, pos: Position) {
        let width = self.text_width();
        let height = self.height().saturating_sub(1) as usize;
        let x = pos.as_usize_x().min(width);
        let y = pos.as_usize_y().min(height);
        let off_x = pos.as_usize_x().saturating_sub(width);
        let off_y = pos.as_usize_y().saturating_sub(height);
        self.scroll_offset = Position::new(off_x, off_y);
        self.set_cursor(Position::new(x, y));
    }

    #[must_use]
    pub fn text_width(&self) -> usize {
        self.dimensions
            .as_usize_x()
            .saturating_sub(self.line_number_width())
    }

    #[must_use]
    pub fn cursor_file(&self) -> Position {
        self.cursor + self.scroll_offset
    }

    #[must_use]
    pub fn cursor_screen(&self) -> Position {
        self.cursor + self.offset()
    }

    pub fn scroll_down(&mut self, lines: usize) {
        if lines + self.scroll_offset.as_usize_y() + self.cursor.as_usize_y()
            < self.buffer.borrow().len_lines()
        {
            self.scroll_offset.add_to_y(lines);
            self.adjust_cursor_x();
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset.sub_to_y(lines);
        self.adjust_cursor_x()
    }

    pub fn move_cursor_down(&mut self, lines: usize) {
        if self.cursor.as_usize_y() >= (self.height() - 1) as usize {
            self.scroll_down(lines);
        } else if self.cursor_file().as_usize_y()
            < self.buffer.borrow().len_lines().saturating_sub(1)
        {
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
            .borrow()
            .line(self.cursor_file().as_usize_y())
            .chars()
            .collect::<String>();
        let mut line_len = line.len();
        if let Mode::Normal = self.mode {
            if line.ends_with('\n') {
                line_len = line_len.saturating_sub(2);
            } else if self.buffer.borrow().len_lines() == self.cursor_file().as_usize_y() {
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
        let buffer = self.buffer.borrow().next_jump_idx(&pos);
        if let Some(i) = buffer {
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
        let buffer = self.buffer.borrow().prev_jump_idx(&pos);
        if let Some(i) = buffer {
            self.cursor.set_x(i);
            self.max_cursor.set_x(self.cursor.as_usize_x());
        } else {
            self.move_cursor_up(1);
            self.end();
            let pos = self.cursor + self.scroll_offset;
            if let Some(i) = self.buffer.borrow().prev_jump_idx(&pos) {
                self.cursor.set_x(i);
                self.max_cursor.set_x(self.cursor.as_usize_x());
            }
        }
    }

    pub fn move_cursor_right(&mut self, cols: usize) {
        if self.cursor.as_usize_x() >= self.text_width() - 1 {
            self.scroll_right(cols)
        } else {
            self.cursor.add_to_x(cols);
            self.max_cursor.set_x(self.cursor.as_usize_x());
            self.adjust_cursor_x();
        }
    }

    pub fn scroll_right(&mut self, cols: usize) {
        self.scroll_offset.add_to_x(cols);
        self.adjust_cursor_x()
        // if cols + self.scroll_offset.as_usize_x() + self.cursor.as_usize_x()
        //     < self
        //         .buffer
        //         .borrow()
        //         .line_len(self.cursor_file().as_usize_y())
        // {
        //     self.scroll_offset.add_to_x(cols);
        //     // self.adjust_cursor_x()
        // }
    }

    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
        // self.move_cursor_down(1);
        self.cursor.add_to_y(1);
        self.cursor.set_x(self.max_cursor.as_usize_x());
        self.cursor.set_x(0);
        self.scroll_offset.set_x(0);
        self.adjust_cursor_x();
    }

    pub fn first_char_in_line(&mut self) {
        let y = (self.cursor + self.scroll_offset).as_usize_y();
        for (i, c) in self.buffer.borrow().line(y).chars().enumerate() {
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
        let total_y = self.buffer.borrow().len_lines().saturating_sub(1);
        // Gets screen height but it also is off by one so we subtract one.
        let screen_y = (self.height() - 1).min(total_y as u16) as usize;
        // Finds Y offset into file but it is off by one as well for indexing so we
        // subtract one as well
        let offset_y = total_y.saturating_sub(screen_y).saturating_sub(1);
        self.cursor.set_y(screen_y);
        self.scroll_offset.set_y(offset_y);
        self.adjust_cursor_x()
    }

    pub fn backspace(&mut self) {
        if self.cursor_file().as_u16() == (0, 0) {
            return;
        }

        let line_index = self
            .buffer
            .borrow()
            .line_to_char(self.cursor_file().as_usize_y());
        let index = line_index + self.cursor_file().as_usize_x() - 1;
        self.buffer.borrow_mut().remove(index..index + 1);

        let new_line = self.buffer.borrow().char_to_line(index);
        if new_line != self.cursor_file().as_usize_y() {
            self.move_cursor_up(1);
        }
        let total = index - self.buffer.borrow().line_to_char(new_line);
        let cursor = total.min(self.text_width().saturating_sub(1));
        let offset = total.saturating_sub(cursor);
        self.scroll_offset.set_x(offset);
        self.cursor.set_x(cursor);
    }

    pub fn delete(&mut self) {
        let index = self
            .buffer
            .borrow()
            .line_to_char(self.cursor_file().as_usize_y())
            + self.cursor_file().as_usize_x();
        if index < self.buffer.borrow().len_chars() {
            self.buffer.borrow_mut().remove(index..index + 1);
        }
        self.adjust_cursor_x();
    }

    pub fn insert_char(&mut self, c: char) {
        let (x, y) = self.cursor_file().as_usize();
        let line_index = self.buffer.borrow().line_to_char(y);
        self.buffer.borrow_mut().insert_char(line_index + x, c);
        self.move_cursor_right(1);
    }

    pub fn delete_line(&mut self) -> String {
        let y = self.cursor_file().as_usize_y();
        let start_idx = self.buffer.borrow().line_to_char(y);
        let end_idx = self.buffer.borrow().line_to_char(y + 1);

        // Grab line to be deleted and return it.
        let line = self.buffer.borrow().line(y);
        // Remove the line...
        self.buffer.borrow_mut().remove(start_idx..end_idx);
        self.adjust_cursor_x();
        line
    }

    pub fn home(&mut self) {
        self.cursor.set_x(0);
        self.scroll_offset.set_x(0);
        self.max_cursor.set_x(0);
    }

    pub fn end(&mut self) {
        let y = self.cursor_file().as_usize_y();
        let line_len = self.buffer.borrow().line_len(y);
        let cursor = line_len.min(self.text_width() - 1);
        let offset = if line_len >= self.text_width() - 1 {
            line_len.saturating_sub(cursor)
        } else {
            0
        };
        self.cursor.set_x(cursor);
        self.scroll_offset.set_x(offset);
        self.max_cursor.set_x(self.cursor.as_usize_x());
        self.adjust_cursor_x();
    }

    // This need to return a Result
    pub fn save(&self) {
        let name = self.buffer.borrow().name().clone();
        if let Some(filename) = name {
            let file =
                std::fs::File::create(filename).expect("Problem opening the file for saving");

            let buff = BufWriter::new(file);

            self.buffer
                .borrow_mut()
                .write_to(buff)
                .expect("Failed to write to file.");
        }
    }

    #[must_use]
    pub fn get_status_bar(&self) -> Option<((u16, u16), Vec<String>)> {
        // FIXME: I hate this so much
        if self.status_bar_state {
            let y = (self.position().as_u16_y() + self.height()) as usize;
            let pos = Position::new(self.position().as_usize_x(), y);

            let left = format!(
                " {} | {}{}",
                self.mode,
                self.buffer()
                    .name()
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string()),
                if self.buffer().is_dirty() { "[+]" } else { "" },
            );
            let right = format!(
                "file: {} | window: {} ",
                self.cursor_file(),
                self.cursor_screen(),
            );
            let middle = (0..(self.text_width().saturating_sub(left.len() + right.len())))
                .map(|_| ' ')
                .collect::<String>();
            return Some((pos.as_u16(), vec![format!("{}{}{}", left, middle, right)]));
        }
        None
    }

    #[must_use]
    pub fn get_line_number(&self) -> Option<((u16, u16), Vec<String>)> {
        // TODO: Pull this out of Window
        if self.line_number_type != LineNumberKind::None {
            let scroll = self.scroll_offset.as_usize_y();
            let len_lines = self.buffer.borrow().len_lines();
            let cursor = self.cursor.as_usize_y() + scroll;
            let width = self.line_number_width().saturating_sub(1);
            let builder = LineNumberBuilder {
                width,
                height: self.height() as usize,
                line_len: len_lines,
                cursor_pos: cursor,
                window_offset: scroll,
                blank_line: "~".into(),
            };
            return Some((
                self.position().as_u16(),
                self.line_number_type.lines(builder),
            ));
        }
        None
    }

    #[must_use]
    pub fn get_text_field(&self) -> Option<((u16, u16), Vec<String>)> {
        // I removed this from in ui.
        // It is the responsibility of the window to clean up line of new lines.
        // .strip_suffix("\r\n").unwrap_or(line)
        let top = self.scroll_offset.as_usize_y();
        let bottom = self.dimensions.as_usize_y() + top;
        let window = self.buffer.borrow().on_screen(top, bottom);
        let formated_window = format_screen(
            &window,
            self.scroll_offset.as_usize_x(),
            self.text_width(),
            self.height() as usize,
        );
        Some((
            self.offset().as_u16(),
            formated_window
                .split("\r\n")
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        ))
    }
}

// Private Methods
impl Window {
    /// Width of `LineNumber` Area if populated.
    fn line_number_width(&self) -> usize {
        if self.line_number_type == LineNumberKind::None {
            return 0;
        }
        self.buffer.borrow().len_lines().to_string().len().max(3) + 2
    }
}

use revi_ui::{
    cursor,
    layout::{Rect, Size, Stack},
    queue, style,
    widget::{BoxWidget, Widget},
};

use std::io::Stdout;

impl Widget for Window {
    fn width(&self) -> u16 {
        self.dimensions.as_u16_x()
    }
    fn height(&self) -> u16 {
        self.dimensions.as_u16_y()
    }
    fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
        if let Some(((x, offset_y), text)) = self.get_text_field() {
            let x = bounds.x();
            let y = bounds.y();
            for (idx, line) in text.iter().enumerate() {
                let y = offset_y + idx as u16;
                queue!(stdout, cursor::MoveTo(x, y), style::Print(line),)
                    .expect("Drawing Window Failed.");
            }
        }
    }
}

impl From<Window> for BoxWidget {
    fn from(widget: Window) -> Self {
        BoxWidget::new(widget)
    }
}
