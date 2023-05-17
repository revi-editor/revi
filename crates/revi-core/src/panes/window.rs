use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use revi_ui::{
    tui::{
        container::Container,
        layout::{Pos, Rect, Size, Stack},
        text::Text,
    },
    Attribute, Color,
};

use super::{
    BufferBounds, BufferMut, Cursor, CursorMovement, CursorPos, Pane, PaneBounds, Scrollable,
};
use crate::{Buffer, Mode};

#[derive(Debug)]
pub struct Window {
    pos: Pos,
    size: Size,
    buffer: Rc<RefCell<Buffer>>,
    has_line_numbers: bool,
    has_status_bar: bool,
    active: bool,
    mode: Mode,
    closing: bool,
}

impl Window {
    const NUMBER_LINE_WIDTH: u16 = 4;
    pub fn new(pos: Pos, size: Size, buffer: Rc<RefCell<Buffer>>) -> Self {
        Self {
            pos,
            size,
            buffer,
            has_line_numbers: false,
            has_status_bar: false,
            active: false,
            mode: Mode::Normal,
            closing: false,
        }
    }

    pub fn with_line_numbers(mut self, flag: bool) -> Self {
        self.has_line_numbers = flag;
        let mut cursor = self.buffer.borrow_mut().cursor;
        cursor.pos.x += flag as u16 * Self::NUMBER_LINE_WIDTH;
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
        let buffer = self.buffer.borrow();
        let has_status_bar = self.has_status_bar as u16;
        let has_line_numbers = self.has_line_numbers as u16;
        let line_text_width = buffer.line_len(y as usize) as u16;
        let pos = Pos {
            x: self.pos.x + (has_line_numbers * Self::NUMBER_LINE_WIDTH),
            y: self.pos.y,
        };
        let pane_height = self.size.height - has_status_bar - 1;
        let buffer_height = buffer.get_rope().len_lines().saturating_sub(2) as u16;
        let height = pane_height.min(buffer_height);
        let size = Size {
            //NOTE: we subtracte 2 from width for offseting the new line
            width: (line_text_width + pos.x).saturating_sub(2).max(pos.x),
            height,
        };
        Rect::with_position(pos, size)
    }

    fn view_contents(&self) -> Text {
        let Size { height, width } = self.size;
        let buffer = self.buffer.borrow();
        let contents = buffer
            .on_screen(height)
            .iter()
            .map(ToString::to_string)
            .chain(std::iter::repeat("\n".into()))
            .take(height as usize)
            .collect::<String>();
        Text::new(&contents)
            .max_width(width)
            .max_height(height)
            .with_comment("text file")
    }

    fn view_status_bar(&self) -> Text {
        let cursor = self.buffer.borrow().cursor;
        let x = cursor.pos.x + cursor.scroll.x;
        let y = cursor.pos.y + cursor.scroll.y;
        let Size { width, .. } = self.size;
        // BUG: this should work
        // let mode = format!("{:-<7}", self.mode);
        // assert_eq!(mode.len(), 7);
        // |Normal| README.md: 5/0
        // eprintln!("len: {}", mode.len());
        let mut mode = self.mode.to_string();
        if mode.len() < 7 {
            mode += &" ".repeat(7 - mode.len());
        }

        Text::new(&format!(
            "{} {}: {}/{}                 ",
            mode,
            self.buffer.borrow().name,
            x,
            y
        ))
        .max_height(1)
        .max_width(width)
        .with_bg(Color::DarkGrey)
        .with_atter(vec![Attribute::Bold].as_slice())
        .with_comment("status bar")
    }

    fn view_line_numbers(&self) -> Text {
        let Size { height, .. } = self.size;
        let height = height - (self.has_status_bar as u16);
        let cursor = self.buffer.borrow().cursor;
        let start = cursor.scroll.y;
        let end = height + cursor.scroll.y;
        let content_rows = (self.buffer.borrow().len_lines() - 2) as u16;
        let text = &(start..=end.min(content_rows))
            .map(|n| format!(" {} \n", n))
            .chain(std::iter::repeat("~\n".into()))
            .take(end as usize)
            .collect::<String>();
        Text::new(text)
            .max_width(Self::NUMBER_LINE_WIDTH)
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
            .with_child(window);

        if self.has_status_bar {
            let status_bar = self.view_status_bar();
            view.push(status_bar);
        }
        view.into()
    }

    fn update(&mut self, mode: Mode, _: revi_ui::Keys) {
        self.mode = mode;
    }

    fn cursor(&self) -> Option<Pos> {
        let cursor = self.buffer.borrow().cursor;
        let x =
            (cursor.pos.x + self.pos.x) + (self.has_line_numbers as u16 * Self::NUMBER_LINE_WIDTH);
        let y = cursor.pos.y + self.pos.y;
        let pos = Pos { x, y };
        Some(pos)
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn set_focused(&mut self, flag: bool) {
        self.active = flag;
    }
    fn close(&self) -> bool {
        self.closing
    }
}

impl CursorPos for Window {
    fn get_cursor_pos(&self) -> Option<Ref<'_, Cursor>> {
        Some(Ref::map(self.buffer.borrow(), |b| &b.cursor))
    }

    fn get_cursor_pos_mut(&mut self) -> Option<RefMut<'_, Cursor>> {
        Some(RefMut::map(self.buffer.borrow_mut(), |b| &mut b.cursor))
    }

    fn get_line_above_bounds(&self) -> Option<Rect> {
        let cursor = self.buffer.borrow().cursor;
        if cursor.pos.y == 0 {
            return None;
        }
        let cursor = self.buffer.borrow().cursor;
        Some(self.create_cursor_bounds(cursor.pos.y + cursor.scroll.y - 1))
    }

    fn get_line_below_bounds(&self) -> Option<Rect> {
        let cursor = self.buffer.borrow().cursor;
        if cursor.pos.y + 1 > self.size.height {
            return None;
        }
        Some(self.create_cursor_bounds(cursor.pos.y + cursor.scroll.y + 1))
    }
}

impl PaneBounds for Window {
    fn get_pane_bounds(&self) -> Option<Rect> {
        let cursor = self.buffer.borrow().cursor;
        Some(self.create_cursor_bounds(cursor.pos.y + cursor.scroll.y))
    }
}

impl BufferBounds for Window {
    fn get_buffer_bounds(&self) -> Option<Size> {
        let Size { height, .. } = self.size;
        let buffer = self.buffer.borrow();
        let text = buffer.on_screen(height);
        let width = text
            .iter()
            .map(|i| i.len_chars() as u16)
            .max()
            .unwrap_or_default();
        let height = buffer.get_rope().len_lines() as u16;
        Some(Size { width, height })
    }
}

impl BufferMut for Window {
    fn set_buffer(&mut self, buf: Rc<RefCell<Buffer>>) {
        self.buffer = buf;
    }
    fn insert_char(&mut self, ch: char) {
        let cursor = self.buffer.borrow().cursor;
        let col = (cursor.pos.x as usize)
            - (self.has_line_numbers as u16 * Self::NUMBER_LINE_WIDTH) as usize;
        let row = cursor.pos.y as usize;
        let mut buffer = self.buffer.borrow_mut();
        let rope = buffer.get_rope_mut();
        let idx = rope.line_to_char(row);
        rope.insert_char(idx + col, ch);
    }
    fn clear_buffer(&mut self) {
        unimplemented!("clear buffer contents")
    }
    fn get_buffer_contents(&self) -> String {
        unimplemented!("get buffer contents")
    }
    fn backspace(&mut self) {
        let cursor = self.buffer.borrow().cursor;
        let col = (cursor.pos.x as usize)
            - (self.has_line_numbers as u16 * Self::NUMBER_LINE_WIDTH) as usize;
        let row = cursor.pos.y as usize;
        let mut buffer = self.buffer.borrow_mut();
        let rope = buffer.get_rope_mut();
        let idx = rope.line_to_char(row);
        let start = (idx + col).saturating_sub(1);
        let end = idx + col;
        rope.remove(start..end);
    }

    fn delete(&mut self) {
        let cursor = self.buffer.borrow().cursor;
        let mut col = (cursor.pos.x as usize)
            - (self.has_line_numbers as u16 * Self::NUMBER_LINE_WIDTH) as usize;
        col += 1;
        let row = cursor.pos.y as usize;
        let mut buffer = self.buffer.borrow_mut();
        let rope = buffer.get_rope_mut();
        let idx = rope.line_to_char(row);
        let start = (idx + col).saturating_sub(1);
        let end = idx + col;
        rope.remove(start..end);
    }

    fn delete_line(&mut self) {
        let mut buffer = self.buffer.borrow_mut();
        let cursor = buffer.cursor;
        let row = cursor.pos.y as usize;
        let rope = buffer.get_rope_mut();
        let start = rope.line_to_char(row);
        let end = rope.line_to_char(row + 1);
        rope.remove(start..end);
    }
}

impl Scrollable for Window {}
impl CursorMovement for Window {}
