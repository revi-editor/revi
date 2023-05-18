use revi_ui::tui::{
    container::Container,
    layout::{Pos, Rect, Size, Stack},
    text::Text,
};

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use super::{
    BufferBounds, BufferMut, Cursor, CursorMovement, CursorPos, Pane, PaneBounds, Scrollable,
};
use crate::{Buffer, Mode};

#[derive(Debug, Default)]
pub struct CommandBar {
    pos: Pos,
    buffer: Rc<RefCell<Buffer>>,
    size: Size,
    active: bool,
    closing: bool,
}

impl CommandBar {
    pub fn new(pos: Pos, width: u16) -> Self {
        Self {
            pos,
            size: Size { height: 1, width },
            ..Default::default()
        }
    }
}

impl Pane for CommandBar {
    fn view(&self) -> revi_ui::tui::widget::BoxWidget {
        // HACK: this should be handled by the render.
        let w = self.size.width - self.active as u16;
        let content = if self.active {
            let w = w as usize;
            format!(
                "{:<w$}",
                self.buffer
                    .borrow()
                    .on_screen(1)
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>()
            )
        } else {
            " ".repeat(self.size.width as usize)
        };
        let text_in_bar = Text::new(content.as_str()).max_width(w).max_height(1);
        let mut view = Container::new(
            Rect::with_position(self.pos, self.size),
            Stack::Horizontally,
        );
        if self.active {
            let colon = Text::new(":").max_height(1).max_width(1);
            view.push(colon);
        }
        view.push(text_in_bar);
        view.into()
    }

    fn update(&mut self, _mode: Mode, _keys: revi_ui::Keys) {}

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

impl CursorPos for CommandBar {
    fn get_cursor_pos(&self) -> Option<Ref<'_, Cursor>> {
        Some(Ref::map(self.buffer.borrow(), |b| &b.cursor))
    }

    fn get_cursor_pos_mut(&mut self) -> Option<RefMut<'_, Cursor>> {
        Some(RefMut::map(self.buffer.borrow_mut(), |b| &mut b.cursor))
    }
}

impl PaneBounds for CommandBar {
    fn get_pane_bounds(&self) -> Option<Rect> {
        let buffer = self.buffer.borrow();
        let size = Size {
            width: buffer.get_rope().len_chars() as u16,
            height: 1,
        };
        Some(Rect::new(size))
    }
}

impl BufferBounds for CommandBar {
    fn get_buffer_bounds(&self) -> Option<Size> {
        let buffer = self.buffer.borrow();
        Some(Size {
            width: buffer.get_rope().len_chars() as u16,
            height: 1,
        })
    }
}

impl BufferMut for CommandBar {
    fn set_buffer(&mut self, _buf: Rc<RefCell<Buffer>>) {
        todo!("set buffer for command bar")
    }
    fn insert_char(&mut self, c: char) {
        let cursor = self.buffer.borrow().cursor;
        let col = cursor.pos.x as usize;
        let row = cursor.pos.y as usize;
        let mut buffer = self.buffer.borrow_mut();
        let rope = buffer.get_rope_mut();
        let idx = rope.line_to_char(row);
        rope.insert_char(idx + col, c);
    }

    fn clear_buffer(&mut self) {
        self.buffer.borrow_mut().clear();
    }

    fn get_buffer_contents(&self) -> String {
        self.buffer.borrow().get_rope().chars().collect::<String>()
    }

    fn backspace(&mut self) {
        let cursor = self.buffer.borrow().cursor;
        let col = cursor.pos.x as usize;
        let row = cursor.pos.y as usize;
        let mut buffer = self.buffer.borrow_mut();
        let rope = buffer.get_rope_mut();
        let idx = rope.line_to_char(row);
        let start = (idx + col).saturating_sub(1);
        let end = idx + col;
        rope.remove(start..end);
        buffer.cursor.pos.x = buffer.cursor.pos.x.saturating_sub(1);
    }
}

impl Scrollable for CommandBar {}
impl CursorMovement for CommandBar {}
