use std::{cell::RefCell, rc::Rc};

use revi_ui::tui::{
    container::Container,
    layout::{Pos, Rect, Size, Stack},
    text::Text,
};

use crate::{
    pane::{Cursor, CursorMovement, Scrollable},
    Buffer, Pane,
};

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

    pub fn text_field_size(&self) -> Size {
        let width = self.size.width - (self.has_line_numbers as u16 * Self::NUMBER_LINE_WIDTH);
        let height = self.size.height - self.has_status_bar as u16;
        Size { width, height }
    }

    fn create_cursor_bounds(&self, y: u16) -> Rect {
        let has_status_bar = self.has_status_bar as u16;
        let has_line_numbers = self.has_line_numbers as u16;
        let line_text_width = self.buffer.borrow().line_len(y as usize) as u16;
        let pos = Pos {
            x: self.pos.x + (has_line_numbers * Self::NUMBER_LINE_WIDTH),
            y: self.pos.y,
        };
        let size = Size {
            //NOTE: we subtracte 2 from width for offseting the new line
            width: (line_text_width + pos.x).saturating_sub(2).max(pos.x),
            height: self.size.height - has_status_bar - 1,
        };
        Rect::with_position(pos, size)
    }

    fn view_contents(&self) -> Text {
        let Size { height, .. } = self.size;
        let top = self.cursor.scroll.y as usize;
        let bottom = (self.cursor.scroll.y + height) as usize;
        let buffer = self.buffer.borrow();
        let contents = buffer
            .on_screen(top, bottom)
            .iter()
            .map(ToString::to_string)
            .collect::<String>();
        Text::new(&contents).with_comment("text file")
    }

    fn view_status_bar(&self) -> Text {
        let x = self.cursor.pos.x + self.cursor.scroll.x;
        let y = self.cursor.pos.y + self.cursor.scroll.y;
        Text::new(&format!(
            "Normal Mode, {}: {}/{}                 ",
            self.buffer.borrow().name,
            x,
            y
        ))
        .max_height(1)
        .with_comment("status bar")
    }

    fn view_line_numbers(&self) -> Text {
        let Size { height, .. } = self.size;
        let height = height - (self.has_status_bar as u16);
        let start = self.cursor.scroll.y;
        let end = height + self.cursor.scroll.y;
        Text::new(
            &(start..=end)
                .map(|n| format!(" {} \n", n))
                .collect::<String>(),
        )
        .max_width(4)
        .with_comment("numbers")
    }
}

impl Pane for Window {
    fn view(&self) -> revi_ui::tui::widget::BoxWidget {
        let Size { width, height } = self.size;
        let text_field = self.view_contents();

        let height = height - (self.has_status_bar as u16);
        let mut window = Container::new(Rect::new(Size::new(width, height)), Stack::Horizontally);

        if self.has_line_numbers {
            let line_numbers = self.view_line_numbers();
            window.push(line_numbers);
        }
        window.push(text_field);

        let mut view = Container::new(Rect::new(self.size), Stack::Vertically)
            .with_comment("everything")
            .stack(Stack::Vertically)
            .with_child(window);

        if self.has_status_bar {
            let status_bar = self.view_status_bar();
            view.push(status_bar);
        }
        view.into()
    }

    fn update(&mut self, _: revi_ui::Keys) {}
}

impl CursorMovement for Window {
    fn get_cursor_bounds(&self) -> Option<Rect> {
        Some(self.create_cursor_bounds(self.cursor.pos.y + self.cursor.scroll.y))
    }

    fn get_line_above_bounds(&self) -> Option<Rect> {
        if self.cursor.pos.y == 0 {
            return None;
        }
        Some(self.create_cursor_bounds(self.cursor.pos.y + self.cursor.scroll.y - 1))
    }

    fn get_line_below_bounds(&self) -> Option<Rect> {
        if self.cursor.pos.y + 1 > self.size.height {
            return None;
        }
        Some(self.create_cursor_bounds(self.cursor.pos.y + self.cursor.scroll.y + 1))
    }
}

impl Scrollable for Window {
    fn get_cursor_pos(&self) -> Option<&Cursor> {
        Some(&self.cursor)
    }

    fn get_cursor_pos_mut(&mut self) -> Option<&mut Cursor> {
        Some(&mut self.cursor)
    }

    fn get_cursor_and_bounds(&self) -> Option<Size> {
        let top = self.cursor.scroll.y as usize;
        let bottom = (self.cursor.scroll.y + self.text_field_size().height) as usize;
        let buffer = self.buffer.borrow();
        let text = buffer.on_screen(top, bottom);
        let width = text
            .iter()
            .map(|i| i.len_chars() as u16)
            .max()
            .unwrap_or_default();
        let height = buffer.rope.len_lines() as u16;
        Some(Size { width, height })
    }
}
