use revi_ui::tui::{
    container::Container,
    layout::{Pos, Rect, Size, Stack},
    text::Text,
};

use crate::{
    pane::{BufferBounds, BufferMut, Cursor, CursorMovement, CursorPos, PaneBounds, Scrollable},
    Mode, Pane,
};

#[derive(Debug, Clone, Default)]
pub struct CommandBar {
    pos: Pos,
    cursor: Cursor,
    size: Size,
    content: String,
    active: bool,
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
            format!("{:<w$}", self.content)
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
}

impl CursorPos for CommandBar {
    fn get_cursor_pos(&self) -> Option<&Cursor> {
        Some(&self.cursor)
    }

    fn get_cursor_pos_mut(&mut self) -> Option<&mut Cursor> {
        Some(&mut self.cursor)
    }
}

impl PaneBounds for CommandBar {
    fn get_pane_bounds(&self) -> Option<Rect> {
        let size = Size {
            width: self.content.len() as u16,
            height: 1,
        };
        Some(Rect::new(size))
    }
}

impl BufferBounds for CommandBar {
    fn get_buffer_bounds(&self) -> Option<Size> {
        Some(Size {
            width: self.content.len() as u16,
            height: 1,
        })
    }
}

impl BufferMut for CommandBar {
    fn insert_char(&mut self, c: char) {
        let idx = self.cursor.pos.x as usize;
        if idx < self.content.len() {
            self.content.insert(idx, c);
            return;
        }
        self.content.push(c);
    }
    fn clear_buffer(&mut self) {
        self.content.clear();
    }
    fn get_buffer_contents(&self) -> String {
        self.content.clone()
    }
    fn backspace(&mut self) {
        let idx = self.cursor.pos.x as usize;
        if idx == self.content.len() {
            self.content.pop();
            return;
        }
        self.content.remove(idx.saturating_sub(1));
    }
}

impl Scrollable for CommandBar {}
impl CursorMovement for CommandBar {}
