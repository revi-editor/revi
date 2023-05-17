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
    Attribute,
};

use super::{
    BufferBounds, BufferMut, Cursor, CursorMovement, CursorPos, Pane, PaneBounds, Scrollable,
};
use crate::{Buffer, Mode};

#[derive(Debug)]
pub struct MessageBox {
    pos: Pos,
    size: Size,
    buffer: Rc<RefCell<Buffer>>,
    footer: String,
    active: bool,
    closing: bool,
}

impl MessageBox {
    pub fn new(pos: Pos, mut size: Size, buffer: Rc<RefCell<Buffer>>) -> Self {
        size.height += 1;
        Self {
            pos,
            size,
            buffer,
            footer: String::new(),
            active: false,
            closing: false,
        }
    }

    pub fn with_footer(mut self, msg: impl Into<String>) -> Self {
        self.footer = msg.into();
        self
    }

    fn create_cursor_bounds(&self, y: u16) -> Rect {
        let line_text_width = self.buffer.borrow().line_len(y as usize) as u16;
        let pos = self.pos;
        let pane_height = self.size.height;
        let buffer_height = self
            .buffer
            .borrow()
            .get_rope()
            .len_lines()
            .saturating_sub(2) as u16;
        let height = pane_height.min(buffer_height);
        let size = Size {
            //NOTE: we subtracte 2 from width for offseting the new line
            width: (line_text_width + pos.x).saturating_sub(2).max(pos.x),
            height,
        };
        Rect::with_position(pos, size)
    }
}

impl Pane for MessageBox {
    fn view(&self) -> revi_ui::tui::widget::BoxWidget {
        let Size { height, width } = self.size;
        let buffer = self.buffer.borrow();
        let contents = buffer
            .on_screen(height)
            .iter()
            .map(ToString::to_string)
            .collect::<String>();
        let text = Text::new(&contents)
            .max_width(width)
            .with_comment("text file");
        use revi_ui::Color;
        let msg = self
            .footer
            .chars()
            .chain(std::iter::repeat(' '))
            .take(width as usize)
            .collect::<String>();
        let bar = Text::new(&msg)
            .with_bg(Color::DarkGrey)
            .with_fg(Color::Black)
            .with_atter(vec![Attribute::Bold, Attribute::Italic].as_slice());
        Container::new(Rect::with_position(self.pos, self.size), Stack::Vertically)
            .with_child(text)
            .with_child(bar)
            .into()
    }

    fn update(&mut self, _: Mode, keys: revi_ui::Keys) {
        let revi_ui::Keys::Key(revi_ui::Key::Null) = keys else {
            return;
        };
        self.closing = true;
    }
    fn cursor(&self) -> Option<Pos> {
        None
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

impl CursorPos for MessageBox {
    fn get_cursor_pos(&self) -> Option<Ref<'_, Cursor>> {
        Some(Ref::map(self.buffer.borrow(), |b| &b.cursor))
    }

    fn get_cursor_pos_mut(&mut self) -> Option<RefMut<'_, Cursor>> {
        Some(RefMut::map(self.buffer.borrow_mut(), |b| &mut b.cursor))
    }

    fn get_line_above_bounds(&self) -> Option<Rect> {
        let cursor = self.buffer.borrow().cursor;
        let pos = cursor.pos.y + cursor.scroll.y;
        if pos == 0 {
            return None;
        }
        Some(self.create_cursor_bounds(pos - 1))
    }

    fn get_line_below_bounds(&self) -> Option<Rect> {
        let buffer_height = self
            .buffer
            .borrow()
            .get_rope()
            .len_lines()
            .saturating_sub(2) as u16;
        let height = self.size.height.min(buffer_height);
        let cursor = self.buffer.borrow().cursor;
        let pos = cursor.pos.y + cursor.scroll.y;
        if pos >= height {
            return None;
        }
        Some(self.create_cursor_bounds(pos + 1))
    }
}

impl PaneBounds for MessageBox {
    fn get_pane_bounds(&self) -> Option<Rect> {
        let cursor = self.buffer.borrow().cursor;
        Some(self.create_cursor_bounds(cursor.pos.y + cursor.scroll.y))
    }
}

impl BufferBounds for MessageBox {
    fn get_buffer_bounds(&self) -> Option<Size> {
        let buffer = self.buffer.borrow();
        let text = buffer.on_screen(self.size.height);
        let width = text
            .iter()
            .map(|i| i.len_chars() as u16)
            .max()
            .unwrap_or_default();
        let height = buffer.get_rope().len_lines() as u16;
        Some(Size { width, height })
    }
}

impl BufferMut for MessageBox {
    fn set_buffer(&mut self, buf: Rc<RefCell<Buffer>>) {
        self.buffer = buf;
    }
    fn insert_char(&mut self, _: char) {}
    fn clear_buffer(&mut self) {}
    fn get_buffer_contents(&self) -> String {
        "".into()
    }
}

impl Scrollable for MessageBox {}
impl CursorMovement for MessageBox {}
