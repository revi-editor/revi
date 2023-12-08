use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

use revi_ui::{
    tui::{
        layout::{Pos, Rect, Size},
        widget::BoxWidget,
    },
    Keys,
};

use crate::Buffer;
use crate::Mode;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cursor {
    pub pos: Pos,
    pub max: Pos,
    pub scroll: Pos,
}

pub trait Pane: Debug + CursorMovement + Scrollable + BufferMut {
    fn view(&self) -> BoxWidget;
    fn update(&mut self, mode: Mode, keys: Keys);
    fn cursor(&self) -> Option<Pos> {
        None
    }
    fn set_focused(&mut self, _: bool);
    fn is_active(&self) -> bool;
    fn close(&self) -> bool;
}

pub trait CursorPos {
    fn get_cursor_pos(&self) -> Option<Ref<'_, Cursor>> {
        None
    }
    fn get_cursor_pos_mut(&mut self) -> Option<RefMut<'_, Cursor>> {
        None
    }
    fn get_line_above_bounds(&self) -> Option<Rect> {
        None
    }
    fn get_line_below_bounds(&self) -> Option<Rect> {
        None
    }
}

pub trait PaneBounds {
    fn get_pane_bounds(&self) -> Option<Rect> {
        None
    }
}

pub trait BufferBounds {
    fn get_buffer_bounds(&self) -> Option<Size> {
        None
    }
}

pub trait CursorMovement: CursorPos + PaneBounds + Scrollable {
    fn move_cursor_up(&mut self) {
        let Some(bounds) = self.get_pane_bounds() else {
            return;
        };
        let above = self.get_line_above_bounds();
        {
            let Some(mut cursor) = self.get_cursor_pos_mut() else {
                return;
            };

            cursor.max.x = cursor.pos.x.max(cursor.max.x);
            if let Some(above) = above {
                cursor.pos.x = cursor.max.x.min(above.width);
            }
            if cursor.pos.y > bounds.y {
                cursor.pos.y -= 1;
                return;
            }
        }
        self.scroll_up();
    }
    fn move_cursor_down(&mut self) {
        let Some(bounds) = self.get_pane_bounds() else {
            return;
        };
        let below = self.get_line_below_bounds();
        {
            let Some(mut cursor) = self.get_cursor_pos_mut() else {
                return;
            };
            cursor.max.x = cursor.pos.x.max(cursor.max.x);
            if let Some(below) = below {
                cursor.pos.x = cursor.max.x.min(below.width.min(cursor.max.x));
            }
            if cursor.pos.y < bounds.height {
                cursor.pos.y += 1;
                return;
            }
        }
        self.scroll_down();
    }
    fn move_cursor_left(&mut self) {
        let Some(mut cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        cursor.pos.x = cursor.pos.x.saturating_sub(1);
        cursor.max.x = cursor.pos.x.min(cursor.max.x);
    }
    fn move_cursor_right(&mut self) {
        let Some(bounds) = self.get_pane_bounds() else {
            return;
        };
        let Some(mut cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        cursor.pos.x = cursor.pos.x.saturating_add(1).min(bounds.width);
        cursor.max.x = cursor.pos.x.max(cursor.max.x);
    }
    fn move_cursor_home(&mut self) {
        let Some(mut cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        cursor.pos.x = 0;
        cursor.max.x = 0;
    }
    fn move_cursor_top_of_buffer(&mut self) {
        let Some(mut cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        cursor.pos.y = 0;
        cursor.max.y = 0;
    }
    fn move_cursor_bottom_of_buffer(&mut self) {
        let Some(b) = self.get_buffer_bounds() else {
            return;
        };
        let Some(mut cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        cursor.pos.y = b.height;
        cursor.max.y = b.height;
    }
}

pub trait Scrollable: BufferBounds + CursorPos {
    fn scroll_up(&mut self) {
        self.get_cursor_pos_mut().map(|mut c| {
            c.scroll.y = c.scroll.y.saturating_sub(1);
            c
        });
    }
    fn scroll_down(&mut self) {
        let bounds = self.get_buffer_bounds();
        let Some(mut cursor) = self.get_cursor_pos_mut() else {
            return;
        };
        let Some(bounds) = bounds else {
            return;
        };
        if cursor.scroll.y + cursor.pos.y < bounds.height.saturating_sub(2) {
            cursor.scroll.y += 1;
        }
    }
    fn scroll_left(&mut self) {}
    fn scroll_right(&mut self) {}
}

pub trait BufferMut {
    fn set_buffer(&mut self, buf: Rc<RefCell<Buffer>>);
    fn get_buffer_contents(&self) -> String;
    fn insert_char(&mut self, c: char);
    fn clear_buffer(&mut self);
    fn backspace(&mut self) {}
    fn delete(&mut self) {}
    fn delete_line(&mut self) {}
}
