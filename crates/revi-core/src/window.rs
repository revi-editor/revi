use std::{cell::RefCell, rc::Rc};

use revi_ui::tui::{
    container::Container,
    layout::{Pos, Rect, Size, Stack},
    text::Text,
    widget::Widget,
};

use crate::{pane::{Cursor, CursorMovement, Scrollable}, Buffer, Pane};

#[derive(Debug)]
pub struct Window {
    pos: Pos,
    cursor: Cursor,
    size: Size,
    buffer: Rc<RefCell<Buffer>>,
    has_line_numbers: bool,
    has_status_bar: bool,
}

impl Window {
    const NUMBER_LINE_WIDTH: u16 = 4;
    pub fn new(pos: Pos, size: Size, buffer: Rc<RefCell<Buffer>>) -> Self {
        Self {
            pos,
            cursor: Cursor::default(),
            size,
            buffer,
            has_line_numbers: false,
            has_status_bar: false,
        }
    }

    pub fn with_line_numbers(mut self, flag: bool) -> Self {
        self.has_line_numbers = flag;
        self.cursor.pos.x += flag as u16 * Self::NUMBER_LINE_WIDTH;
        self
    }

    pub fn with_status_bar(mut self, flag: bool) -> Self {
        self.has_status_bar = flag;
        self
    }

    fn create_cursor_bounds(&self, y: u16) -> Rect {
        let has_status_bar = self.has_status_bar as u16;
        let has_line_numbers = self.has_line_numbers as u16;
        let line_text_width = self.buffer.borrow().line_len(y as usize) as u16 - 1;
        let pos = Pos {
            x: self.pos.x + (has_line_numbers * Self::NUMBER_LINE_WIDTH),
            y: self.pos.y,
        };
        let size = Size {
            width: (line_text_width + pos.x).saturating_sub(1),
            height: self.size.height - has_status_bar - 1,
        };
        Rect::with_position(pos, size)
    }
}

impl Pane for Window {
    fn view(&self) -> revi_ui::tui::widget::BoxWidget {
        let Size { width, height } = self.size;
        let top = 0;
        let bottom = height as usize;
        let buffer = self.buffer.borrow();
        let contents = buffer.on_screen(top, bottom);
        let text_field = Text::new(&contents.to_string()).with_comment("text file");

        let window = Container::new(Rect::new(Size::new(width, height - (self.has_status_bar as u16))), Stack::Horizontally)
            .with_comment("window with text file and numbers");

        let window = if self.has_line_numbers {
            let text_numbers = Text::new(
                &(1..=window.width())
                    .map(|n| format!(" {} \n", n))
                    .collect::<String>(),
            )
            .max_width(4)
            .with_comment("numbers");
            window.push(text_numbers).push(text_field)
        } else {
            window.push(text_field)
        };


        let file_name = &buffer.name;

        let view = Container::new(Rect::new(self.size), Stack::Vertically)
            .with_comment("everything")
            .stack(Stack::Vertically)
            .push(window);
        let view = if self.has_status_bar {
            view.push(
                Text::new(&format!(
                    "Normal Mode, {}: ({:?}|{:?}) line_len: {}             ",
                    file_name,
                    self.get_cursor_pos().unwrap(),
                    self.cursor,
                    self.buffer.borrow().line_len(self.cursor.pos.y as usize) as u16,
                ))
                .max_height(1)
                .with_comment("status bar"))
        } else {
            view
        };
        view.into()
    }

    fn update(&mut self, _: revi_ui::Keys) {}
}

impl CursorMovement for Window {
    fn get_cursor_pos(&self) -> Option<&Cursor> {
        Some(&self.cursor)
    }

    fn get_cursor_pos_mut(&mut self) -> Option<&mut Cursor> {
        Some(&mut self.cursor)
    }

    fn get_cursor_bounds(&self) -> Option<Rect> {
        Some(self.create_cursor_bounds(self.cursor.pos.y))
    }

    fn get_line_above_bounds(&self) -> Option<Rect> {
        if self.cursor.pos.y == 0 {
            return None;
        }
        Some(self.create_cursor_bounds(self.cursor.pos.y - 1))
    }

    fn get_line_below_bounds(&self) -> Option<Rect> {
        if self.cursor.pos.y + 1 > self.size.height {
            return None;
        }
        Some(self.create_cursor_bounds(self.cursor.pos.y + 1))
    }
}

impl Scrollable for Window {}
